# struct_to_array

Convert between homogeneous structs (all fields the same type) and fixed-size arrays.

## Usage

```rust
use struct_to_array::StructToArray;

#[derive(StructToArray)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    
    // Convert to array
    let arr = point.to_arr();
    assert_eq!(arr, [1.0, 2.0, 3.0]);
    
    // Convert back from array
    let point = Point3D::from_arr(arr);
    assert_eq!(point.x, 1.0);
}
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

## License

MIT OR Apache-2.0

## LLM Notice
This was written to my spec, but entirely by LLM. It could contain errors, but my spec included a mandate for quite a lot of property-based testing. I've only looked through this quickly, but the proptests look good and cover a huge variety of cases, way more than I would have ever had the patience to write myself.

This is good enough for me, perhaps for you too :-)

But if you find any mistakes, please open an issue or PR!