#![no_std]
#![feature(allocator_api)]

pub mod slab;
pub mod cache;

#[cfg(test)]
mod tests;

pub use cache::SlabCache;
pub use slab::Slab;
