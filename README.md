# Slab Allocator - Implémentation Rust no_std

Allocateur slab en Rust pour environnement `no_std`, conçu pour la gestion mémoire au niveau noyau.

## 📚 Théorie du Slab Allocator

Le slab allocator est une technique de gestion mémoire développée par Jeff Bonwick pour le noyau Solaris, aujourd'hui utilisée dans Linux et de nombreux systèmes d'exploitation.

### Principe de fonctionnement

Le slab allocator organise la mémoire en trois niveaux hiérarchiques :

1. **Caches** : Pools d'objets d'une taille spécifique
2. **Slabs** : Blocs de mémoire contiguë (typiquement 4KB) contenant plusieurs objets pré-alloués
3. **Objets** : Unités de mémoire de taille fixe au sein des slabs

### Avantages

- **Performance O(1)** : Allocation et désallocation en temps constant
- **Réduction de la fragmentation** : Les objets de même taille sont regroupés
- **Optimisation du cache CPU** : Les objets réutilisés restent "chauds" dans le cache
- **Pas de réinitialisation** : Les objets peuvent conserver leur état entre allocations
- **Économie mémoire** : Pas de métadonnées par objet

### Gestion des listes

Chaque cache maintient trois listes de slabs :

- **Partiels** : Slabs avec des objets libres disponibles → utilisés en priorité
- **Pleins** : Slabs complètement occupés → mis de côté temporairement
- **Vides** : Slabs sans objets alloués → gardés en réserve pour réutilisation

## 🏗️ Architecture du projet

```
slab-allocator/
├── Cargo.toml              # Configuration du projet Rust
├── Authors.md              # Informations sur les auteurs
├── README.md               # Ce fichier
├── .gitignore              # Fichiers ignorés par Git
└── src/
    ├── lib.rs             # Point d'entrée de la bibliothèque
    ├── slab.rs            # Implémentation d'un slab individuel
    ├── cache.rs           # Gestion du cache de slabs
    └── tests.rs           # Tests unitaires
```

## 🔧 Compilation et tests

### Prérequis

- Rust (stable ou nightly) : https://rustup.rs/
- Environnement Windows avec MSVC ou Linux/macOS

### Compilation

```bash
cargo build              # Compilation en mode debug
cargo build --release    # Compilation optimisée
```

### Tests

```bash
cargo test               # Lance tous les tests
cargo test -- --nocapture --test-threads=1  # Tests avec affichage détaillé
cargo test test_debug_detaille -- --nocapture  # Test détaillé spécifique
```

### Vérification du code

```bash
cargo fmt --check        # Vérification du formatage
cargo clippy             # Analyse statique
```

## 📦 Modules

### `slab.rs` - Gestion d'un slab

**Structure `Slab`**
```rust
pub struct Slab {
    prochain: Option<NonNull<Slab>>,  // Liste chaînée de slabs
    utilises: usize,                   // Nombre d'objets alloués
    liste_libre: u32,                  // Index du premier objet libre
    memoire: NonNull<u8>,              // Pointeur vers la mémoire
    taille_objet: usize,               // Taille d'un objet en bytes
    nb_objets: usize,                  // Nombre total d'objets
}
```

**Fonctionnalités :**
- `nouveau()` : Initialise un slab avec une liste libre chaînée
- `allouer()` : Retourne un objet libre en O(1)
- `liberer()` : Remet un objet en tête de liste libre
- `est_vide()` : Vérifie si tous les objets sont libres
- `est_plein()` : Vérifie si aucun objet n'est disponible
- `objet_appartient()` : Détermine si un pointeur appartient à ce slab

### `cache.rs` - Cache de slabs

**Structure `SlabCache`**
```rust
pub struct SlabCache {
    taille_objet: usize,
    objets_par_slab: usize,
    partiels: Option<NonNull<Slab>>,  // Slabs avec places libres
    pleins: Option<NonNull<Slab>>,    // Slabs pleins
    vides: Option<NonNull<Slab>>,     // Slabs vides
}
```

**Fonctionnalités :**
- `nouveau()` : Crée un cache pour une taille d'objet donnée
- `allouer()` : Alloue depuis un slab partiel ou vide
- `liberer()` : Libère et réorganise les listes de slabs
- Gestion automatique des transitions entre listes (partiel ↔ plein ↔ vide)

## 🧪 Tests

Le projet contient 5 tests unitaires :

### 1. `test_creation_cache`
Vérifie la création correcte d'un cache avec taille d'objet spécifique.

### 2. `test_slab_basique`
Teste l'allocation et la libération basique d'un objet.

### 3. `test_allocations_multiples`
Alloue 10 objets, les libère tous, et vérifie l'absence de fuite mémoire.

### 4. `test_slab_plein`
Remplit complètement un slab et vérifie le refus d'allocation supplémentaire.

### 5. `test_debug_detaille`
Test complet avec affichage détaillé montrant :
- Allocations avec adresses mémoire
- Libérations d'objets
- Réutilisation de mémoire (même adresse réutilisée)
- Gestion du remplissage complet
- Vérification absence de fuite mémoire

**Exemple de sortie :**
```
╔══════════════════════════════════════════════════╗
║  TEST DÉTAILLÉ - SLAB ALLOCATOR                 ║
╚══════════════════════════════════════════════════╝

✓ Slab créé: 5 objets de 64 bytes chacun

→ Allocation objet 1...
  ✓ Objet 1 alloué à l'adresse: 0xf1550fd418

→ Libération objet 2...
  ✓ Objet 2 libéré

→ Allocation objet 4 (va réutiliser l'espace de obj2)...
  ✓ RÉUTILISATION CONFIRMÉE! Même adresse.

╔══════════════════════════════════════════════════╗
║  ✓ TEST RÉUSSI - AUCUNE FUITE MÉMOIRE           ║
╚══════════════════════════════════════════════════╝
```

## 🔐 Sécurité et documentation

### Blocs `unsafe`

Tous les blocs `unsafe` sont documentés avec des sections `# Safety` expliquant :
- Les invariants requis
- Les conditions de validité
- Les responsabilités de l'appelant

**Exemple :**
```rust
/// # Safety
/// `memoire` doit pointer vers TAILLE_SLAB bytes valides et alignés.
/// Le pointeur doit rester valide pendant toute la durée de vie du slab.
pub unsafe fn nouveau(memoire: NonNull<u8>, ...) -> Self
```

## 📊 Résultats des tests

```bash
running 5 tests
test tests::test_allocations_multiples ... ok
test tests::test_creation_cache ... ok
test tests::test_debug_detaille ... ok
test tests::test_slab_basique ... ok
test tests::test_slab_plein ... ok

test result: ok. 5 passed; 0 failed
```

## 🎓 Contexte pédagogique

Ce projet démontre :
- Maîtrise de Rust en environnement `no_std`
- Compréhension des structures de données bas niveau
- Gestion manuelle de la mémoire avec `unsafe`
- Documentation rigoureuse du code dangereux
- Tests unitaires complets
- Utilisation professionnelle de Git

## 📚 Références

- [The Slab Allocator: An Object-Caching Kernel Memory Allocator (Bonwick, 1994)](https://people.eecs.berkeley.edu/~kubitron/courses/cs194-24-S14/hand-outs/bonwick_slab.pdf)
- [Linux Kernel Slab Allocator Documentation](https://www.kernel.org/doc/gorman/html/understand/understand011.html)
- [Learning Rust with Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Rust `no_std` Documentation](https://docs.rust-embedded.org/book/intro/no-std.html)

## 👥 Auteurs

**Idris BOUDOUR** - idrisbr52@gmail.com  
**Ameri Ibrahim GUINDO**

GitHub: [IdrisBr/slab-allocator](https://github.com/IdrisBr/slab-allocator)

## 📄 Licence

Projet académique - ESGI Master Cybersécurité
