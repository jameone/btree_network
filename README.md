# Binary Tree Network (btree_network)

## Branch Status

| build                 | test               | main               |
| :-------------------: | :----------------: | :----------------: |
| ![CodeBuild(build)]   | ![CodeBuild(test)] | ![CodeBuild(main)] |

![CodeBuild(main)]
[![Version badge]][crates.io]
[![Docs badge]][docs.rs]

[CodeBuild(main)]: https://codebuild.us-east-1.amazonaws.com/badges?uuid=eyJlbmNyeXB0ZWREYXRhIjoidm9wRDJYSzlzMm02RDhvTllnRXBaTXRBclZzaEhQREVySnZiMjVybzQ1QTJOODBiY3VKUzg0WmR4bzFJV3p6N3JOL1lEMitMT1RZTGNkQ3JtblZqam9FPSIsIml2UGFyYW1ldGVyU3BlYyI6IklCWVc3T01oZFJSQnVwRWIiLCJtYXRlcmlhbFNldFNlcmlhbCI6MX0%3D&branch=main
[CodeBuild(test)]: https://codebuild.us-east-1.amazonaws.com/badges?uuid=eyJlbmNyeXB0ZWREYXRhIjoidm9wRDJYSzlzMm02RDhvTllnRXBaTXRBclZzaEhQREVySnZiMjVybzQ1QTJOODBiY3VKUzg0WmR4bzFJV3p6N3JOL1lEMitMT1RZTGNkQ3JtblZqam9FPSIsIml2UGFyYW1ldGVyU3BlYyI6IklCWVc3T01oZFJSQnVwRWIiLCJtYXRlcmlhbFNldFNlcmlhbCI6MX0%3D&branch=main
[CodeBuild(build)]: https://codebuild.us-east-1.amazonaws.com/badges?uuid=eyJlbmNyeXB0ZWREYXRhIjoidm9wRDJYSzlzMm02RDhvTllnRXBaTXRBclZzaEhQREVySnZiMjVybzQ1QTJOODBiY3VKUzg0WmR4bzFJV3p6N3JOL1lEMitMT1RZTGNkQ3JtblZqam9FPSIsIml2UGFyYW1ldGVyU3BlYyI6IklCWVc3T01oZFJSQnVwRWIiLCJtYXRlcmlhbFNldFNlcmlhbCI6MX0%3D&branch=main
[Version badge]: https://img.shields.io/crates/v/btree_network
[crates.io]: https://crates.io/crates/btree_network
[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-blue
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
    let mut network: BTreeNetwork<String> = BTreeNetwork::new();
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
btree_network = "0.2.3"
```

## API

Please see the [API](src/network/api.rs) for a full list of
available methods.

## License

This work is dually licensed under MIT OR Apache-2.0.