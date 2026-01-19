#![cfg(test)]

extern crate std;
use std::println;

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

#[test]
fn test_debug_detaille() {
    unsafe {
        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║  TEST DÉTAILLÉ - SLAB ALLOCATOR                 ║");
        println!("╚══════════════════════════════════════════════════╝\n");

        let mut buffer = [0u8; 4096];
        let ptr = NonNull::new(buffer.as_mut_ptr()).unwrap();
        let mut slab = Slab::nouveau(ptr, 64, 5);

        println!("✓ Slab créé: 5 objets de 64 bytes chacun");
        println!("  Mémoire totale: {} bytes\n", 5 * 64);

        println!("─────────────────────────────────────────────────");
        println!("📦 PHASE 1: Allocations");
        println!("─────────────────────────────────────────────────\n");

        println!("→ Allocation objet 1...");
        let obj1 = slab.allouer().unwrap();
        println!("  ✓ Objet 1 alloué à l'adresse: {:?}", obj1);
        println!("  État: Slab vide? {}\n", if slab.est_vide() { "OUI" } else { "NON" });

        println!("→ Allocation objet 2...");
        let obj2 = slab.allouer().unwrap();
        println!("  ✓ Objet 2 alloué à l'adresse: {:?}", obj2);

        println!("→ Allocation objet 3...");
        let obj3 = slab.allouer().unwrap();
        println!("  ✓ Objet 3 alloué à l'adresse: {:?}\n", obj3);

        println!("─────────────────────────────────────────────────");
        println!("🔄 PHASE 2: Libération");
        println!("─────────────────────────────────────────────────\n");

        println!("→ Libération objet 2...");
        slab.liberer(obj2);
        println!("  ✓ Objet 2 libéré");
        println!("  État: Slab plein? {}\n", if slab.est_plein() { "OUI" } else { "NON" });

        println!("─────────────────────────────────────────────────");
        println!("♻️  PHASE 3: Réutilisation");
        println!("─────────────────────────────────────────────────\n");

        println!("→ Allocation objet 4 (va réutiliser l'espace de obj2)...");
        let obj4 = slab.allouer().unwrap();
        println!("  ✓ Objet 4 alloué à l'adresse: {:?}", obj4);
        println!("  Note: obj2 était à {:?}", obj2);
        if obj4 == obj2 {
            println!("  ✓ RÉUTILISATION CONFIRMÉE! Même adresse.\n");
        }

        println!("─────────────────────────────────────────────────");
        println!("🧪 PHASE 4: Remplissage complet");
        println!("─────────────────────────────────────────────────\n");

        println!("→ Allocation objets 5 et 6...");
        let obj5 = slab.allouer().unwrap();
        println!("  ✓ Objet 5 alloué: {:?}", obj5);
        let obj6 = slab.allouer().unwrap();
        println!("  ✓ Objet 6 alloué: {:?}", obj6);

        println!("\n  État: Slab plein? {}", if slab.est_plein() { "OUI ✓" } else { "NON" });

        println!("\n→ Tentative d'allocation alors que le slab est plein...");
        let obj_fail = slab.allouer();
        match obj_fail {
            None => println!("  ✓ Allocation refusée (None) - Comportement correct!"),
            Some(_) => println!("  ✗ ERREUR: Ne devrait pas allouer!"),
        }

        println!("\n─────────────────────────────────────────────────");
        println!("🧹 PHASE 5: Nettoyage complet");
        println!("─────────────────────────────────────────────────\n");

        println!("→ Libération de tous les objets...");
        slab.liberer(obj1);
        slab.liberer(obj3);
        slab.liberer(obj4);
        slab.liberer(obj5);
        slab.liberer(obj6);
        println!("  ✓ Tous les objets libérés");

        println!("\n  État final: Slab vide? {}", if slab.est_vide() { "OUI ✓" } else { "NON ✗" });

        assert!(slab.est_vide());

        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║  ✓ TEST RÉUSSI - AUCUNE FUITE MÉMOIRE           ║");
        println!("╚══════════════════════════════════════════════════╝\n");
    }
}
