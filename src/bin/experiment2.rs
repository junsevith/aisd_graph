use std::sync::{Arc, Mutex};
use std::time::Instant;

use rand::{Rng, thread_rng};
use rayon::prelude::*;

use graph::chart::draw_chart;
use graph::graph::algorithms::{broadcast_path, prim, random_full};
use graph::indicator::Indicator;

fn main() {
    let n_min = 20;
    let n_max = 2000;
    let n_step = 20;
    let rep = 20;
    let rep2 = 20;

    let iter = (n_min..=n_max).step_by(n_step);

    let indicator = Arc::new(Mutex::new(Indicator::new_linear(iter.clone())));

    let experiment_start = Instant::now();

    let (avg, (min, max)): (Vec<_>, (Vec<_>, Vec<_>)) = iter.clone().collect::<Vec<_>>().into_par_iter()
        .map(move |n| {
            let (sum, min, max) = (0..rep2).into_par_iter().map(move |_rep| {
                let graph = random_full(n);
                let tree = prim(&graph);

                let (sum, min, max) = (0..rep).into_par_iter().map(move |_rep| {

                    let start = thread_rng().gen_range(0..n);
                    broadcast_path(&tree, start).0

                }).fold(
                    || (0, usize::MAX, 0),
                    |(sum, min, max), x| (sum + x, min.min(x), max.max(x)),
                ).reduce(
                    || (0, usize::MAX, 0),
                    |(a, b, c), (d, e, f)| (a + d, b.min(e), c.max(f)),
                );

                (sum as f64 / rep as f64, min as f64, max as f64)

            }).fold(
                || (0., f64::MAX, 0f64),
                |(sum, min, max), (a, b, c)| (sum + a, min.min(b), max.max(c)),
            ).reduce(
                || (0., f64::MAX, 0f64),
                |(a, b, c), (d, e, f)| (a + d, b.min(e), c.max(f)),
            );

            indicator.lock().unwrap().done(n);
            println!("{}", indicator.lock().unwrap());

            (sum / rep2 as f64, (min, max))
        }).unzip();

    let experiment_time = experiment_start.elapsed().as_secs_f64();
    println!("Experiment time: {:.2} s", experiment_time);

    draw_chart(vec![avg, min, max], vec!["Average", "Minimum", "Maximum"], iter, "Tree broadcast", |_, y| y);
}