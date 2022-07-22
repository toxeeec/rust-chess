use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_chess::chess::Game;

const KIWI_POS: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const STARTING_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn perft_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("depth 4");
    group.sample_size(20);

    group.bench_function("kiwi", |b| {
        b.iter(|| Game::perft(black_box(KIWI_POS), black_box(4)));
    });
    group.bench_function("starting", |b| {
        b.iter(|| Game::perft(black_box(STARTING_POS), black_box(4)));
    });

    group.finish();
}

criterion_group!(benches, perft_benchmark);
criterion_main!(benches);
