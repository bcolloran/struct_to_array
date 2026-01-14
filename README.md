# struct_to_array

Convert between homogeneous structs (all fields the same type) and fixed-size arrays.

## Usage

```rust
use struct_to_array::StructToArray;

#[derive(StructToArray)]
#[struct_to_array(crate = "struct_to_array")]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };

// Convert to array
let arr = point.to_arr();
assert_eq!(arr, [1.0, 2.0, 3.0]);

// Convert back from array
let point = Point3D::from_arr(arr);
assert_eq!(point.x, 1.0);
```

## Features

- **Named field structs**: `struct Point { x: T, y: T }`
- **Tuple structs**: `struct Pair(T, T)`
- **Generic types**: Works with any uniform field type
- **No `Copy` required**: Value-based conversions work with move-only types
- **Vec conversion**: Optional `StructToVec` trait for `Vec` conversions

## Requirements

All fields must have identical type tokens. The macro checks this at compile time.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
struct_to_array = { path = "struct_to_array" }
```

## More Examples

### Tuple Structs

```rust
use struct_to_array::StructToArray;

#[derive(StructToArray)]
#[struct_to_array(crate = "struct_to_array")]
struct Pair(i32, i32);

let pair = Pair(10, 20);
let arr = pair.to_arr();
assert_eq!(arr, [10, 20]);

let reconstructed = Pair::from_arr([30, 40]);
assert_eq!(reconstructed.0, 30);
assert_eq!(reconstructed.1, 40);
```

### Generic Types

```rust
use struct_to_array::StructToArray;

#[derive(StructToArray)]
#[struct_to_array(crate = "struct_to_array")]
struct GenericPair<T> {
    first: T,
    second: T,
}

let string_pair = GenericPair {
    first: "hello".to_string(),
    second: "world".to_string(),
};
let arr = string_pair.to_arr();
assert_eq!(arr[0], "hello");
assert_eq!(arr[1], "world");
```

### Vec Conversion

```rust
use struct_to_array::{StructToArray, StructToVec};

#[derive(StructToArray)]
#[struct_to_array(crate = "struct_to_array")]
struct Point2D {
    x: f32,
    y: f32,
}

let point = Point2D { x: 3.0, y: 4.0 };

// Convert to Vec
let vec = point.to_vec();
assert_eq!(vec, vec![3.0, 4.0]);

// Convert from Vec
let point2 = Point2D::from_vec(&vec);
assert_eq!(point2.x, 3.0);
assert_eq!(point2.y, 4.0);
```

### Non-Copy Types

The trait works with move-only types that don't implement `Copy`:

```rust
use struct_to_array::StructToArray;

#[derive(StructToArray)]
#[struct_to_array(crate = "struct_to_array")]
struct VecPair {
    first: Vec<i32>,
    second: Vec<i32>,
}

let pair = VecPair {
    first: vec![1, 2, 3],
    second: vec![4, 5, 6],
};

let arr = pair.to_arr();
assert_eq!(arr[0], vec![1, 2, 3]);
assert_eq!(arr[1], vec![4, 5, 6]);

let reconstructed = VecPair::from_arr(arr);
assert_eq!(reconstructed.first, vec![1, 2, 3]);
```

## License

MIT OR Apache-2.0

## LLM Notice
This was written to my spec, but entirely by LLM. It could contain errors, but my spec included a mandate for quite a lot of property-based testing. I've only looked through this quickly, but the proptests look good and cover a huge variety of cases, way more than I would have ever had the patience to write myself.

This is good enough for me, perhaps for you too :-)

But if you find any mistakes, please open an issue or PR!