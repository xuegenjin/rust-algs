pub use self::uf::UF;

pub use self::quick_find::QuickFind;
pub use self::quick_union::QuickUnion;
pub use self::weighted_quick_union::WeightedQuickUnion;

#[macro_use]
pub mod uf;

pub mod quick_find;
pub mod quick_union;
pub mod weighted_quick_union;
