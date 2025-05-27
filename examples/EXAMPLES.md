# Running Nav Examples

This document explains how to run the examples included with the Nav crate, particularly those that require specific features.

## Array Transformations Example

The `array_transformations` example demonstrates how to use the Nav crate with the `array` feature to perform geometric transformations on 2D arrays.

### Prerequisites

- The `array` feature must be enabled

### Running the Example

You can run the example using:

```bash
cargo run --example array_transform --features array
```

### Expected Output

```txt
[[1, 2, 3],
 [4, 5, 6],
 [7, 8, 9]]

Identity
[[1, 2, 3],
 [4, 5, 6],
 [7, 8, 9]]

Rotate90
[[7, 4, 1],
 [8, 5, 2],
 [9, 6, 3]]

Rotate180
[[9, 8, 7],
 [6, 5, 4],
 [3, 2, 1]]

Rotate270
[[3, 6, 9],
 [2, 5, 8],
 [1, 4, 7]]

FlipHorizontal
[[3, 2, 1],
 [6, 5, 4],
 [9, 8, 7]]

FlipDiagonal
[[1, 4, 7],
 [2, 5, 8],
 [3, 6, 9]]

FlipVertical
[[7, 8, 9],
 [4, 5, 6],
 [1, 2, 3]]

FlipAntiDiagonal
[[9, 6, 3],
 [8, 5, 2],
 [7, 4, 1]]
```
