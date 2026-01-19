use crate::slab::Slab;
use core::ptr::NonNull;
use core::mem;

const TAILLE_SLAB: usize = 4096;
const TAILLE_MIN_OBJET: usize = mem::size_of::<u32>();

pub struct SlabCache {
    taille_objet: usize,
    #[allow(dead_code)]
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
                    self.partiels = slab.prochain;
                    slab.prochain = self.pleins;
                    self.pleins = Some(slab_ptr);
                }
                return Some(obj);
            }
        }

        if let Some(mut slab_ptr) = self.vides {
            let slab = slab_ptr.as_mut();
            self.vides = slab.prochain;
            slab.prochain = self.partiels;
            self.partiels = Some(slab_ptr);
            return slab.allouer();
        }

        None
    }

    /// # Safety
    /// `ptr` doit provenir de ce cache et ne plus être utilisé
    pub unsafe fn liberer(&mut self, ptr: NonNull<u8>) {
        if !self.chercher_et_liberer(ptr, true) {
            self.chercher_et_liberer(ptr, false);
        }
    }

    unsafe fn chercher_et_liberer(&mut self, ptr: NonNull<u8>, dans_pleins: bool) -> bool {
        let liste = if dans_pleins {
            &mut self.pleins
        } else {
            &mut self.partiels
        };

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
    }

    pub fn taille_objet(&self) -> usize {
        self.taille_objet
    }
}
