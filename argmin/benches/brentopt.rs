// Copyright 2018-2020 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use criterion::{criterion_group, criterion_main, Criterion};

use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::brent::BrentOpt;

/// Test function: `f(x) = exp(-x) - exp(5-x/2)`
/// xmin == 2 log(2 exp(-5))
/// xmin ~= -8.6137056388801093812
/// f(xmin) == -exp(10)/4
/// f(xmin) ~= -5506.6164487016791292
struct TestFunc {}

impl CostFunction for TestFunc {
    // one dimensional problem, no vector needed
    type Param = f64;
    type Output = f64;

    fn cost(&self, x: &Self::Param) -> Result<Self::Output, Error> {
        Ok((-x).exp() - (5. - x / 2.).exp())
    }
}

fn run() -> Result<(), Error> {
    let cost = TestFunc {};
    let solver = BrentOpt::new(-10., 10.);

    let _res = Executor::new(cost, solver)
        .configure(|state| state.max_iters(100))
        // .add_observer(SlogLogger::term(), ObserverMode::Always)
        .run()
        .unwrap();
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("BrentOpt", |b| {
        b.iter(|| run().expect("Benchmark should run without errors"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
