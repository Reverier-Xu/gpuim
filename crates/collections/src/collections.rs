pub type HashMap<K, V> = FxHashMap<K, V>;
pub type HashSet<T> = FxHashSet<T>;
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;
pub type IndexSet<T> = indexmap::IndexSet<T, rustc_hash::FxBuildHasher>;

pub use std::collections::*;

pub use indexmap::Equivalent;
pub use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

pub mod vecmap;
#[cfg(test)]
mod vecmap_tests;
