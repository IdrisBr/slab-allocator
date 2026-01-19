use crate::slab::Slab;
use core::ptr::NonNull;
use core::mem;
use core::alloc::{GlobalAlloc, Layout};

const TAILLE_SLAB: usize = 4096;
const TAILLE_MIN_OBJET: usize = mem::size_of::<u32>();

pub struct SlabCache {
    taille_objet: usize,
    objets_par_slab: usize,
    partiels: Option<NonNull<Slab>>,
    pleins: Option<NonNull<Slab>>,
    vides: Option<NonNull<Slab>>,
}

impl SlabCache {
    pub const fn nouveau(taille_objet: usize) -> Self {
        let taille = if taille_objet < TAILLE_MIN_OBJET {
            TAILLE_MIN_OBJET
        } else {
            taille_objet
        };

        let objets_par_slab = (TAILLE_SLAB - mem::size_of::<Slab>()) / taille;

        SlabCache {
            taille_objet: taille,
            objets_par_slab,
            partiels: None,
            pleins: None,
            vides: None,
        }
    }

    /// # Safety
    /// Le pointeur retourné doit être libéré avec `liberer`
    pub unsafe fn allouer(&mut self) -> Option<NonNull<u8>> {
        if let Some(mut slab_ptr) = self.partiels {
            let slab = slab_ptr.as_mut();
            if let Some(obj) = slab.allouer() {
                if slab.est_plein() {
                    self.partiels = (*slab).prochain;
                    (*slab).prochain = self.pleins;
                    self.pleins = Some(slab_ptr);
                }
                return Some(obj);
            }
        }

        if let Some(mut slab_ptr) = self.vides {
            let slab = slab_ptr.as_mut();
            self.vides = (*slab).prochain;
            (*slab).prochain = self.partiels;
            self.partiels = Some(slab_ptr);
            return slab.allouer();
        }

        None
    }

    /// # Safety
    /// `ptr` doit provenir de ce cache et ne plus être utilisé
    pub unsafe fn liberer(&mut self, ptr: NonNull<u8>) {
        let mut chercher_dans = |liste: &mut Option<NonNull<Slab>>| -> bool {
            let mut courant = *liste;
            let mut precedent: Option<NonNull<Slab>> = None;

            while let Some(mut slab_ptr) = courant {
                let slab = slab_ptr.as_mut();

                if slab.objet_appartient(ptr) {
                    let etait_plein = slab.est_plein();
                    slab.liberer(ptr);

                    if etait_plein {
                        if let Some(mut prev) = precedent {
                            prev.as_mut().prochain = slab.prochain;
                        } else {
                            *liste = slab.prochain;
                        }

                        slab.prochain = self.partiels;
                        self.partiels = Some(slab_ptr);
                    } else if slab.est_vide() {
                        if let Some(mut prev) = precedent {
                            prev.as_mut().prochain = slab.prochain;
                        } else {
                            *liste = slab.prochain;
                        }

                        slab.prochain = self.vides;
                        self.vides = Some(slab_ptr);
                    }

                    return true;
                }

                precedent = Some(slab_ptr);
                courant = slab.prochain;
            }
            false
        };

        if !chercher_dans(&mut self.pleins) {
            chercher_dans(&mut self.partiels);
        }
    }

    pub fn taille_objet(&self) -> usize {
        self.taille_objet
    }
}

pub struct AllocateurSlab {
    cache_8: SlabCache,
    cache_16: SlabCache,
    cache_32: SlabCache,
    cache_64: SlabCache,
    cache_128: SlabCache,
    cache_256: SlabCache,
    cache_512: SlabCache,
    cache_1024: SlabCache,
}

impl AllocateurSlab {
    pub const fn nouveau() -> Self {
        AllocateurSlab {
            cache_8: SlabCache::nouveau(8),
            cache_16: SlabCache::nouveau(16),
            cache_32: SlabCache::nouveau(32),
            cache_64: SlabCache::nouveau(64),
            cache_128: SlabCache::nouveau(128),
            cache_256: SlabCache::nouveau(256),
            cache_512: SlabCache::nouveau(512),
            cache_1024: SlabCache::nouveau(1024),
        }
    }

    fn selectionner_cache(&mut self, taille: usize) -> Option<&mut SlabCache> {
        match taille {
            0..=8 => Some(&mut self.cache_8),
            9..=16 => Some(&mut self.cache_16),
            17..=32 => Some(&mut self.cache_32),
            33..=64 => Some(&mut self.cache_64),
            65..=128 => Some(&mut self.cache_128),
            129..=256 => Some(&mut self.cache_256),
            257..=512 => Some(&mut self.cache_512),
            513..=1024 => Some(&mut self.cache_1024),
            _ => None,
        }
    }
}

unsafe impl GlobalAlloc for AllocateurSlab {
    /// # Safety
    /// L'allocateur doit être initialisé et le pointeur retourné doit être libéré avec le même layout
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    /// # Safety
    /// `ptr` doit provenir de cet allocateur et `layout` doit correspondre à l'allocation
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}
