use chess5dlib::parse::test::read_and_parse;
use chess5dlib::prelude::*;
use chess5dlib::gen::*;
use chess5dlib::mate::*;
use criterion::measurement::Measurement;
use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
    // BatchSize
};
// use rand::prelude::*;
use std::time::{Duration, Instant};

fn bench_is_mate_sub<M: Measurement>(group: &mut BenchmarkGroup<M>, game: &Game, name: &str) {
    let partial_game = no_partial_game(&game);

    let mut sigma = 0;
    let mut delta = Duration::new(0, 0);

    group.bench_with_input(BenchmarkId::new(name, "is_mate"), game, |b, game| {
        b.iter(|| {
            let start = Instant::now();
            match is_mate(game, &partial_game, Some(Duration::new(10, 0))) {
                Mate::Checkmate => {
                    sigma += 1;
                    delta += start.elapsed();
                }
                Mate::None(_ms, _pos, _gen) => {
                    sigma += 1;
                    delta += start.elapsed();
                }
                x => panic!("Expected checkmate or none, got {:?}", x),
            }
        })
    });

    if sigma > 0 {
        println!("Timelines: {}", game.info.len_timelines());
        println!("Boards to play on: {}", partial_game.own_boards(game).count());
        println!("Time (s): {}", delta.as_millis() as f64 / 1000.0);
        println!("Positions: {}", sigma);
        println!("Position / ms: {}", sigma as f64 / delta.as_millis() as f64);
    }
}

fn bench_gen_legal_moveset_sub<M: Measurement>(group: &mut BenchmarkGroup<M>, game: &Game, name: &str) {
    let partial_game = no_partial_game(&game);

    let mut sigma = 0;
    let mut delta = Duration::new(0, 0);

    // let mut iter = GenLegalMovesetIter::new(game, &partial_game, None);

    group.bench_with_input(BenchmarkId::new(name, "GenLegalMovesetIter"), game, |b, game| {
        b.iter(|| {
            let start = Instant::now();

            // iter.inc();
            // sigma += 1;
            // delta += start.elapsed();
            // println!("Took {} μs", start.elapsed().as_nanos() as f64 / 1000.0);

            let mut iter = GenLegalMovesetIter::new(game, std::borrow::Cow::Borrowed(&partial_game), Some(Duration::new(10, 0)));
            match iter.next() {
                None => {
                    if iter.timed_out() {
                        panic!("Timed out!");
                    } else {
                        sigma += 1;
                        delta += start.elapsed();
                    }
                }
                Some(_ms) => {
                    if iter.timed_out() {
                        panic!("Timed out!");
                    } else {
                        sigma += 1;
                        delta += start.elapsed();
                    }
                }
            }
        })
    });

    if sigma > 0 {
        println!("Timelines: {}", game.info.len_timelines());
        println!("Boards to play on: {}", partial_game.own_boards(game).count());
        println!("Time (s): {}", delta.as_millis() as f64 / 1000.0);
        println!("Positions: {}", sigma);
        println!("Position / ms: {}", sigma as f64 / delta.as_millis() as f64);
    }
}

pub fn bench_checkmate<M: Measurement>(c: &mut Criterion<M>) {
    {
        let mut moveset_group = c.benchmark_group("Checkmates");
        moveset_group
            .warm_up_time(Duration::new(10, 0))
            .measurement_time(Duration::new(60, 0));
        let game = read_and_parse("tests/games/standard-checkmate.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Standard Checkmate 1");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Standard Checkmate 1");
        let game = read_and_parse("tests/games/standard-checkmate-2.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Standard Checkmate 2");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Standard Checkmate 2");
        let game = read_and_parse("tests/games/standard-checkmate-3.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Standard Checkmate 3");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Standard Checkmate 3");
        let game = read_and_parse("tests/games/standard-checkmate-4.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Standard Checkmate 4");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Standard Checkmate 4");
        let game = read_and_parse("tests/games/dead-timeline.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Dead Timeline 1");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Dead Timeline 1");
    }
    {
        let mut moveset_group = c.benchmark_group("Non-mates");
        moveset_group
            .warm_up_time(Duration::new(5, 0))
            .measurement_time(Duration::new(10, 0));
        let game = read_and_parse("tests/games/standard-check.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Standard Check");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Standard Check");
        let game = read_and_parse("tests/games/standard-complex.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Standard Complex");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Standard Complex");
        let game = read_and_parse("tests/games/standard-complex-2.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Standard Complex 2");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Standard Complex 2");
        let game = read_and_parse("tests/games/tricky-nonmate.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Tricky Nonmate");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Tricky Nonmate");
        let game = read_and_parse("tests/games/issue-1.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Issue 1");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Issue 1");
        let game = read_and_parse("tests/games/dead-timeline-2.json");
        bench_is_mate_sub(&mut moveset_group, &game, "Dead Timeline 2");
        bench_gen_legal_moveset_sub(&mut moveset_group, &game, "Dead Timeline 2");
    }
}


criterion_group!(
    name = mate;
    config = Criterion::default();
    targets = bench_checkmate
);
criterion_main!(mate);
