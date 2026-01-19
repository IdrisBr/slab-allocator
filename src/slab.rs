use core::ptr::NonNull;

#[allow(dead_code)]
const TAILLE_SLAB: usize = 4096;

pub struct Slab {
    pub(crate) prochain: Option<NonNull<Slab>>,
    utilises: usize,
    liste_libre: u32,
    memoire: NonNull<u8>,
    taille_objet: usize,
    nb_objets: usize,
}

impl Slab {
    /// # Safety
    /// `memoire` doit pointer vers TAILLE_SLAB bytes valides et alignés
    pub unsafe fn nouveau(memoire: NonNull<u8>, taille_objet: usize, nb_objets: usize) -> Self {
        for i in 0..nb_objets {
            let offset = i * taille_objet;
            let ptr = memoire.as_ptr().add(offset) as *mut u32;
            ptr.write(if i < nb_objets - 1 { (i + 1) as u32 } else { u32::MAX });
        }

        Slab {
            prochain: None,
            utilises: 0,
            liste_libre: 0,
            memoire,
            taille_objet,
            nb_objets,
        }
    }

    /// # Safety
    /// Le slab doit être initialisé et le pointeur retourné ne doit pas dépasser la durée de vie du slab
    pub unsafe fn allouer(&mut self) -> Option<NonNull<u8>> {
        if self.liste_libre == u32::MAX {
            return None;
        }

        let index = self.liste_libre as usize;
        let ptr = self.memoire.as_ptr().add(index * self.taille_objet);
        self.liste_libre = (ptr as *const u32).read();
        self.utilises += 1;

        NonNull::new(ptr)
    }

    /// # Safety
    /// `ptr` doit provenir de ce slab et ne plus être utilisé après
    pub unsafe fn liberer(&mut self, ptr: NonNull<u8>) {
        let offset = ptr.as_ptr().offset_from(self.memoire.as_ptr()) as usize;
        let index = offset / self.taille_objet;

        (ptr.as_ptr() as *mut u32).write(self.liste_libre);
        self.liste_libre = index as u32;
        self.utilises -= 1;
    }

    pub fn est_vide(&self) -> bool {
        self.utilises == 0
    }

    pub fn est_plein(&self) -> bool {
        self.liste_libre == u32::MAX
    }

    pub fn objet_appartient(&self, ptr: NonNull<u8>) -> bool {
        let base = self.memoire.as_ptr() as usize;
        let ptr_addr = ptr.as_ptr() as usize;
        let taille_totale = self.nb_objets * self.taille_objet;

        ptr_addr >= base && ptr_addr < base + taille_totale
    }
}
