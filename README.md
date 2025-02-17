# Out of Bounds Vector Access in Rust

This repository demonstrates a common error in Rust: accessing an element in a vector using an index that is out of bounds.  This leads to a runtime panic, which can be difficult to debug.

The `bug.rs` file contains the erroneous code, while `bugSolution.rs` demonstrates how to handle this error gracefully using safe methods.

## How to Reproduce

1. Clone this repository.
2. Navigate to the directory containing the files.
3. Compile and run `bug.rs` using `rustc bug.rs && ./bug`.  This will result in a runtime panic.
4. Compile and run `bugSolution.rs` using `rustc bugSolution.rs && ./bugSolution`.  This will demonstrate the safe approach.

## Solution

The solution involves using the `.get()` method instead of direct indexing.  `.get()` returns an `Option`, allowing you to handle the case where the index is out of bounds without panicking.