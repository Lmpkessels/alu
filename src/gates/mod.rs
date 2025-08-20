pub mod derived;
pub mod primitives;
pub mod universal;

pub use derived::{xor, xnor};
pub use primitives::{and, or, not};
pub use universal::{nand, nor};