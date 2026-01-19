#![cfg(test)]

use crate::cache::SlabCache;
use crate::slab::Slab;
use core::ptr::NonNull;

#[test]
fn test_creation_cache() {
    let cache = SlabCache::nouveau(64);
    assert_eq!(cache.taille_objet(), 64);
}

#[test]
fn test_slab_basique() {
    unsafe {
        let mut buffer = [0u8; 4096];
        let ptr = NonNull::new(buffer.as_mut_ptr()).unwrap();
        let mut slab = Slab::nouveau(ptr, 64, 32);

        let obj1 = slab.allouer().expect("Allocation echouee");
        assert!(!slab.est_vide());

        slab.liberer(obj1);
        assert!(slab.est_vide());
    }
}

#[test]
fn test_allocations_multiples() {
    unsafe {
        let mut buffer = [0u8; 4096];
        let ptr = NonNull::new(buffer.as_mut_ptr()).unwrap();
        let mut slab = Slab::nouveau(ptr, 64, 32);

        let mut objets = [None; 10];
        for i in 0..10 {
            objets[i] = slab.allouer();
        }

        for obj in objets.iter().flatten() {
            slab.liberer(*obj);
        }

        assert!(slab.est_vide());
    }
}

#[test]
fn test_slab_plein() {
    unsafe {
        let mut buffer = [0u8; 4096];
        let ptr = NonNull::new(buffer.as_mut_ptr()).unwrap();
        let nb_objets = 5;
        let mut slab = Slab::nouveau(ptr, 64, nb_objets);

        for _ in 0..nb_objets {
            slab.allouer().expect("Allocation echouee");
        }

        assert!(slab.est_plein());
        assert!(slab.allouer().is_none());
    }
}
