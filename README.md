# Slab Allocator - Implémentation Rust

Allocateur slab en `no_std` pour une allocation mémoire efficace au niveau noyau.

## Théorie du Slab Allocator

Le slab allocator organise la mémoire en trois niveaux:

1. **Caches**: Pools d'objets d'une taille spécifique
2. **Slabs**: Blocs de mémoire contiguë contenant des objets pré-alloués
3. **Objects**: Unités de mémoire de taille fixe dans les slabs

### Avantages

- Réduit la fragmentation mémoire
- Allocation/désallocation rapide (O(1))
- Optimisation du cache matériel
- Réutilisation d'objets sans réinitialisation

### Fonctionnement

Chaque slab maintient une liste libre d'objets disponibles. Lors d'une allocation, on prend le premier objet libre. Lors d'une désallocation, l'objet retourne en tête de liste.

## Compilation

```bash
cargo build --release
```

## Tests

```bash
cargo test
cargo +nightly miri test  # Vérification du code unsafe
```

## Structure du projet

```
slab-allocator/
├── Cargo.toml
├── Authors.md
├── README.md
├── src/
│   ├── lib.rs
│   ├── slab.rs
│   └── cache.rs
└── .gitignore
```

## Documentation des zones unsafe

Chaque bloc unsafe est documenté avec une section `# Safety` expliquant les invariants requis.

## Références

- [The Slab Allocator: Bonwick 1994](https://people.eecs.berkeley.edu/~kubitron/courses/cs194-24-S14/hand-outs/bonwick_slab.pdf)
- [Linux Kernel Slab Allocator](https://www.kernel.org/doc/gorman/html/understand/understand011.html)
- [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
