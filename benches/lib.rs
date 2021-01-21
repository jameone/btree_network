use criterion::criterion_main;

mod network;
use network::*;

criterion_main!(network_benches,);
