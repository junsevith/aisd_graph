use std::sync::{Arc, Mutex};
use std::time::Instant;
use rayon::prelude::*;
use graph::chart::draw_chart;
use graph::graph::algorithms::{kruskal, prim, random_full};
use graph::indicator::Indicator;

fn main() {
    let n_min = 20;
    let n_max = 2000;
    let n_step = 20;
    let rep = 20;

    let iter = (n_min..=n_max).step_by(n_step);

    let indicator = Arc::new(Mutex::new(Indicator::new_linear(iter.clone())));

    let experiment_start = Instant::now();

    let (kruskal, prim): (Vec<_>, Vec<_>) = iter.clone().collect::<Vec<_>>().into_par_iter()
        .map(move |n| {
            let sum = (0..rep).into_par_iter().map(move |_rep| {
                let graph = random_full(n);

                let start = Instant::now();
                let _ = kruskal(&graph);
                let time1 = start.elapsed().as_nanos();

                let start = Instant::now();
                let _ = prim(&graph);
                let time2 = start.elapsed().as_nanos();

                (time1, time2)
            }).reduce(
                || (0, 0),
                |(a, b), (c, d)| (a + c, b + d),
            );
            indicator.lock().unwrap().done(n);
            println!("{}", indicator.lock().unwrap());
            (sum.0 as f64 / rep as f64, sum.1 as f64 / rep as f64)
        })
        .unzip();

    let experiment_time = experiment_start.elapsed().as_secs_f64();
    println!("Experiment time: {:.2} s", experiment_time);

    draw_chart(vec![kruskal, prim], vec!["Kruskal", "Prim"], iter, "kruskal_vs_prim", |_, y| y);
}