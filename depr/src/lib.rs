//! # struct_to_array
//!
//! Convert between homogeneous structs (all fields the same type) and fixed-size arrays.
//!
//! ## Usage
//!
//! ```
//! use struct_to_array::StructToArray;
//!
//! #[derive(StructToArray)]
//! struct Point3D {
//!     x: f64,
//!     y: f64,
//!     z: f64,
//! }
//!
//! # fn main() {
//! let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
//! 
//! // Convert to array
//! let arr = point.to_arr();
//! assert_eq!(arr, [1.0, 2.0, 3.0]);
//! 
//! // Convert back from array
//! let point = Point3D::from_arr(arr);
//! assert_eq!(point.x, 1.0);
//! # }
//! ```
//!
//! ## Features
//!
//! - **Named field structs**: `struct Point { x: T, y: T }`
//! - **Tuple structs**: `struct Pair(T, T)`
//! - **Generic types**: Works with any uniform field type
//! - **No `Copy` required**: Value-based conversions work with move-only types
//! - **Vec conversion**: Optional `StructToVec` trait for `Vec` conversions
//!
//! ## Requirements
//!
//! All fields must have identical type tokens. The macro checks this at compile time.

/// Convert between a homogeneous struct (all fields of type `Item`) and `[Item; N]`.
///
/// This is intentionally value-based (consumes `self` / takes array by value).
/// That keeps it safe and requires no layout assumptions.
pub trait StructToArray<Item, const N: usize>: Sized {
    /// Number of atomic items of type `Item` in the struct (recursively).
    const N_ATOMS: usize = N;
    fn to_arr(self) -> [Item; N];
    fn from_arr(a: [Item; N]) -> Self;
    #[inline]
    fn num_fields() -> usize {
        assert!(
            N == Self::N_ATOMS,
            "Declared N ({}) does not match actual number of fields ({})",
            N,
            Self::N_ATOMS
        );
        N
    }
}

pub trait StructToVec<Item, const N: usize>: Sized + StructToArray<Item, N>
where
    Item: Clone,
{
    fn to_vec(self) -> Vec<Item>;
    fn from_vec(v: &[Item]) -> Self;
}

impl<S, Item, const N: usize> StructToVec<Item, N> for S
where
    S: StructToArray<Item, N>,
    Item: Clone,
{
    #[inline]
    fn to_vec(self) -> Vec<Item> {
        let arr = self.to_arr();
        arr.to_vec()
    }
    #[inline]
    fn from_vec(v: &[Item]) -> Self {
        assert!(
            v.len() == N,
            "Input vec length {} does not match expected {}",
            v.len(),
            N
        );
        let arr: [Item; N] = std::array::from_fn(|i| v[i].clone());
        Self::from_arr(arr)
    }
}

pub use struct_to_array_derive::StructToArray;
