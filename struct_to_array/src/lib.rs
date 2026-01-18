#![doc = include_str!("../README.md")]

/// A trait for fixed-size arrays that allows accessing the array's length as a constant
/// and provides common array operations.
///
/// This trait is automatically implemented for all `[Item; N]` arrays.
pub trait FixedArray<Item>: Sized {
    /// The length of the array.
    const N: usize;

    /// Returns a slice view of the array.
    fn as_slice(&self) -> &[Item];

    /// Creates a new array by calling the provided function for each index.
    fn from_fn<F: FnMut(usize) -> Item>(f: F) -> Self;
}

impl<Item, const N: usize> FixedArray<Item> for [Item; N] {
    const N: usize = N;

    #[inline]
    fn as_slice(&self) -> &[Item] {
        self
    }

    #[inline]
    fn from_fn<F: FnMut(usize) -> Item>(f: F) -> Self {
        ::std::array::from_fn(f)
    }
}

/// Convert between a homogeneous struct (all fields of type `Item`) and a fixed-size array.
///
/// This is intentionally value-based (consumes `self` / takes array by value).
/// That keeps it safe and requires no layout assumptions.
///
/// The associated type `Arr` specifies the exact array type, which must implement `FixedArray<Item>`.
/// This design allows the array size to be determined by the implementation without requiring
/// unstable Rust features like `generic_const_exprs`.
pub trait StructToArray<Item>: Sized {
    /// The array type used to store the struct's fields.
    type Arr: FixedArray<Item>;

    /// Converts the struct into an array of its fields.
    fn to_arr(self) -> Self::Arr;

    /// Creates a struct from an array of field values.
    fn from_arr(arr: Self::Arr) -> Self;

    /// Returns the number of fields in the struct.
    #[inline]
    fn num_fields() -> usize {
        <Self::Arr as FixedArray<Item>>::N
    }
}

/// Extension trait for converting between structs and `Vec<Item>`.
///
/// This trait is automatically implemented for any type that implements `StructToArray<Item>`
/// where `Item: Clone`.
pub trait StructToVec<Item>: Sized + StructToArray<Item>
where
    Item: Clone,
{
    /// Converts the struct into a `Vec` of its fields.
    fn to_vec(self) -> Vec<Item>;

    /// Creates a struct from a slice of field values.
    ///
    /// # Panics
    ///
    /// Panics if the slice length does not match the number of fields in the struct.
    fn from_vec(v: &[Item]) -> Self;
}

impl<S, Item> StructToVec<Item> for S
where
    S: StructToArray<Item>,
    Item: Clone,
{
    #[inline]
    fn to_vec(self) -> Vec<Item> {
        let arr = self.to_arr();
        arr.as_slice().to_vec()
    }

    #[inline]
    fn from_vec(v: &[Item]) -> Self {
        let expected = <S::Arr as FixedArray<Item>>::N;
        assert!(
            v.len() == expected,
            "Input vec length {} does not match expected {}",
            v.len(),
            expected
        );
        let arr = <S::Arr as FixedArray<Item>>::from_fn(|i| v[i].clone());
        Self::from_arr(arr)
    }
}

pub use struct_to_array_derive::StructToArray;
