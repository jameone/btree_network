# Binary Tree Network (btree_network)

[![Build Status](https://codebuild.us-east-1.amazonaws.com/badges?uuid=eyJlbmNyeXB0ZWREYXRhIjoib1ZNOXdDNDVpM09TOGtZRmUxU083d3pzSlR1K1VQZUQ0MkdDbW5Ldlp6ZFpsdFZVcXFMTktmR3RwWmd2bFVnMGRWOUxzSEdMV1Nqbkw5YUpiL2h3Q2VRPSIsIml2UGFyYW1ldGVyU3BlYyI6ImJiR0U0NmQvMHk3RnUwZUUiLCJtYXRlcmlhbFNldFNlcmlhbCI6MX0%3D&branch=main)](https://codebuild.us-east-1.amazonaws.com/badges?uuid=eyJlbmNyeXB0ZWREYXRhIjoib1ZNOXdDNDVpM09TOGtZRmUxU083d3pzSlR1K1VQZUQ0MkdDbW5Ldlp6ZFpsdFZVcXFMTktmR3RwWmd2bFVnMGRWOUxzSEdMV1Nqbkw5YUpiL2h3Q2VRPSIsIml2UGFyYW1ldGVyU3BlYyI6ImJiR0U0NmQvMHk3RnUwZUUiLCJtYXRlcmlhbFNldFNlcmlhbCI6MX0%3D&branch=main)
[![Code Version](https://img.shields.io/crates/v/btree_network)](https://img.shields.io/crates/v/btree_network)
[![Docs badge]][docs.rs]

[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs.rs]: https://docs.rs/btree_network/

This library is a minimal implementation of a network 
(abstract data structure) by way of a single binary tree map
(`BTreeMap`). This implementation is often referred to as
an adjacency list.

The primary goals of this implementation are to be 
minimal and idiomatic to the Rust language. The `alloc`
crate is the only dependency when compiled with default
features and is not optional. As one might assume, `alloc`
is required for reason the implementation relies on `BTreeMap`
(and the `BTreeSet` wrapper).

## Example
```rust
use crate::BTreeNetwork;

fn main() {
    let mut network: BTreeNetwork<String, String> = BTreeNetwork::new();
    // Add nodes.
    network.add_vertex(String::from("Tarzan"));
    network.add_vertex(String::from("Jane"));
    // Add a relationship.
    network.add_edge(String::from("Tarzan"), String::from("Jane"));
    
    // Assert relationship now exists.
    assert!(network.adjacdent(String::from("Tarzan"), String::from("Jane")));
}
```

## Usage

Add the following to your `Cargo.toml` file:
```toml
[dependencies]
btree_network = "0.1.3"
```

## API

Please see the [API](src/network/api.rs) for a full list of
available methods.

## License

This work is dually licensed under MIT OR Apache-2.0.