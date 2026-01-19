@echo off
echo ============================================
echo   CONFIGURATION GIT - SLAB ALLOCATOR
echo ============================================
echo.

git init
git config user.name "Idris BR"
git config user.email "idrisbr52@gmail.com"

echo [OK] Depot Git initialise
echo.

echo ============================================
echo   COMMITS INITIAUX
echo ============================================
echo.

git add Cargo.toml Authors.md README.md .gitignore
git commit -m "feat: structure initiale du projet slab allocator"
echo [OK] Commit 1/4

git add src/lib.rs src/slab.rs
git commit -m "feat: implementation du module slab avec allocation/liberation"
echo [OK] Commit 2/4

git add src/cache.rs
git commit -m "feat: ajout du cache de slabs et allocateur global"
echo [OK] Commit 3/4

git add src/tests.rs
git commit -m "test: tests unitaires pour slab et cache"
echo [OK] Commit 4/4

echo.
echo ============================================
echo   HISTORIQUE DES COMMITS
echo ============================================
echo.
git log --oneline --graph

echo.
echo ============================================
echo   CREATION DU BUNDLE
echo ============================================
echo.

git bundle create slab-allocator.bundle --all
git bundle verify slab-allocator.bundle

echo.
echo [OK] Bundle cree: slab-allocator.bundle
echo.
echo Projet pret pour soumission!
pause
