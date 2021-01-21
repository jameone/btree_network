use btree_network::error::Error;
use btree_network::*;
use criterion::{black_box, Criterion};

fn setup() -> Result<BTreeNetwork<String>, Error> {
    let mut network: BTreeNetwork<String> = BTreeNetwork::new();
    network.add_vertex(String::from("0"));
    network.add_vertex(String::from("1"));
    network.add_vertex(String::from("2"));
    network.add_vertex(String::from("3"));
    network.add_vertex(String::from("4"));
    network.add_vertex(String::from("5"));
    network.add_vertex(String::from("6"));
    network.add_vertex(String::from("7"));
    network.add_vertex(String::from("8"));
    network.add_vertex(String::from("9"));

    network.add_edge(String::from("0"), String::from("1"))?;
    network.add_edge(String::from("0"), String::from("2"))?;
    network.add_edge(String::from("0"), String::from("3"))?;
    network.add_edge(String::from("0"), String::from("4"))?;
    network.add_edge(String::from("0"), String::from("5"))?;
    network.add_edge(String::from("0"), String::from("6"))?;
    network.add_edge(String::from("0"), String::from("7"))?;
    network.add_edge(String::from("0"), String::from("8"))?;
    network.add_edge(String::from("0"), String::from("9"))?;

    network.add_edge(String::from("1"), String::from("2"))?;
    network.add_edge(String::from("1"), String::from("3"))?;
    network.add_edge(String::from("1"), String::from("4"))?;
    network.add_edge(String::from("1"), String::from("5"))?;
    network.add_edge(String::from("1"), String::from("6"))?;
    network.add_edge(String::from("1"), String::from("7"))?;
    network.add_edge(String::from("1"), String::from("8"))?;
    network.add_edge(String::from("1"), String::from("9"))?;

    network.add_edge(String::from("2"), String::from("3"))?;
    network.add_edge(String::from("2"), String::from("4"))?;
    network.add_edge(String::from("2"), String::from("5"))?;
    network.add_edge(String::from("2"), String::from("6"))?;
    network.add_edge(String::from("2"), String::from("7"))?;
    network.add_edge(String::from("2"), String::from("8"))?;
    network.add_edge(String::from("2"), String::from("9"))?;

    network.add_edge(String::from("3"), String::from("4"))?;
    network.add_edge(String::from("3"), String::from("5"))?;
    network.add_edge(String::from("3"), String::from("6"))?;
    network.add_edge(String::from("3"), String::from("7"))?;
    network.add_edge(String::from("3"), String::from("8"))?;
    network.add_edge(String::from("3"), String::from("9"))?;

    network.add_edge(String::from("4"), String::from("5"))?;
    network.add_edge(String::from("4"), String::from("6"))?;
    network.add_edge(String::from("4"), String::from("7"))?;
    network.add_edge(String::from("4"), String::from("8"))?;
    network.add_edge(String::from("4"), String::from("9"))?;

    network.add_edge(String::from("5"), String::from("6"))?;
    network.add_edge(String::from("5"), String::from("7"))?;
    network.add_edge(String::from("5"), String::from("8"))?;
    network.add_edge(String::from("5"), String::from("9"))?;

    network.add_edge(String::from("6"), String::from("7"))?;
    network.add_edge(String::from("6"), String::from("8"))?;
    network.add_edge(String::from("6"), String::from("9"))?;

    network.add_edge(String::from("7"), String::from("8"))?;
    network.add_edge(String::from("7"), String::from("9"))?;

    Ok(network)
}

pub fn clone_benchmark(c: &mut Criterion) {
    let network = setup().unwrap();
    c.bench_function("network::network clone", |b| {
        b.iter(|| black_box(network.clone()))
    });
}

pub fn vertices_benchmark(c: &mut Criterion) {
    let network = setup().unwrap();
    c.bench_function("network::api::Vertices", |b| {
        b.iter(|| black_box(network.vertices()))
    });
}

pub fn add_vertex_benchmark(c: &mut Criterion) {
    let mut network = setup().unwrap();
    c.bench_function("network::api::AddVertex (vertex does not exist)", |b| {
        b.iter(|| black_box(network.add_vertex(String::from("10"))))
    });

    c.bench_function("network::api::AddVertex (vertex exists)", |b| {
        b.iter(|| black_box(network.add_vertex(String::from("0"))))
    });
}

pub fn add_edge_benchmark(c: &mut Criterion) {
    let mut network = setup().unwrap();
    c.bench_function("network::api::AddEdge (edge does not exist)", |b| {
        b.iter(|| black_box(network.add_edge(String::from("9"), String::from("0"))))
    });

    c.bench_function("network::api::AddEdge (edge exists)", |b| {
        b.iter(|| black_box(network.add_edge(String::from("0"), String::from("1"))))
    });
}

pub fn get_vertex_value_benchmark(c: &mut Criterion) {
    let network = setup().unwrap();
    c.bench_function(
        "network::api::GetVertexValue (vertex does not exist)",
        |b| b.iter(|| black_box(network.get_vertex_value(String::from("10")))),
    );

    c.bench_function("network::api::GetVertexValue (vertex exists)", |b| {
        b.iter(|| black_box(network.get_vertex_value(String::from("0"))))
    });
}

pub fn remove_edge_benchmark(c: &mut Criterion) {
    let mut network = setup().unwrap();
    c.bench_function("network::api::RemoveEdge (edge does not exist)", |b| {
        b.iter(|| black_box(network.remove_edge(String::from("1"), String::from("0"))))
    });

    c.bench_function("network::api::RemoveEdge (edge exists)", |b| {
        b.iter(|| black_box(network.remove_edge(String::from("9"), String::from("8"))))
    });
}

pub fn remove_vertex_benchmark(c: &mut Criterion) {
    let mut network = setup().unwrap();
    c.bench_function("network::api::RemoveVertex (vertex does not exist)", |b| {
        b.iter(|| black_box(network.remove_vertex(String::from("10"))))
    });

    c.bench_function("network::api::RemoveVertex (vertex exists)", |b| {
        b.iter(|| black_box(network.remove_vertex(String::from("0"))))
    });
}

pub fn adjacent_benchmark(c: &mut Criterion) {
    let network = setup().unwrap();
    c.bench_function("network::api::Adjacent (vertices are not adjacent)", |b| {
        b.iter(|| black_box(network.adjacent(String::from("9"), String::from("0"))))
    });

    c.bench_function("network::api::Adjacent (vertices are adjacent)", |b| {
        b.iter(|| black_box(network.adjacent(String::from("0"), String::from("1"))))
    });

    c.bench_function("network::api::Adjacent (vertex does not exist)", |b| {
        b.iter(|| black_box(network.adjacent(String::from("10"), String::from("1"))))
    });
}

pub fn connections_benchmark(c: &mut Criterion) {
    let network = setup().unwrap();
    c.bench_function("network::api::Connections (vertex does not exist)", |b| {
        b.iter(|| black_box(network.connections(String::from("10"))))
    });

    c.bench_function("network::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(network.connections(String::from("0"))))
    });

    c.bench_function("network::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(network.connections(String::from("8"))))
    });

    c.bench_function("network::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(network.connections(String::from("9"))))
    });
}
