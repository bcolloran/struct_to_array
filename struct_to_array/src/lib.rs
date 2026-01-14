#![doc = include_str!("../../README.md")]
//! Value-to-value conversions between a uniform-field struct and a fixed-size array.

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
