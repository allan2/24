use chrono::{Duration, Utc};
use csv::Writer;
use std::{collections::BTreeMap, io};
use twentyfour::{Cards, Op, Ops};

fn main() -> Result<(), io::Error> {
    let max = 13;

    run_dist(max, Ops::default(), "dist_with_pow.csv")?;

    run_dist(
        max,
        Ops::with_ops(vec![Op::Add, Op::Sub, Op::Mul, Op::Div]),
        "dist_without_pow.csv",
    )?;

    Ok(())
}

fn run_dist(max: u8, ops: Ops, outfile: &str) -> Result<(), io::Error> {
    // Map of solution counts and frequency
    let mut map = BTreeMap::new();

    let start_time = Utc::now().time();
    for i in 1..=max {
        for j in 1..=max {
            for k in 1..=max {
                for l in 1..=max {
                    let cards = Cards::with_ops(vec![i, j, k, l], ops.clone());
                    let sols = cards.solve();
                    map.entry(sols.len()).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
    }
    let elapsed = Utc::now().time() - start_time;
    print_elapsed_time(elapsed);

    println!(
        "Mean time per card set: {:.5}s",
        elapsed.num_milliseconds() as f64 / 1000. / (max as f64).powi(4)
    );

    println!("\nSolution Frequency");
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    let mut wtr = Writer::from_path(outfile)?;
    for (k, v) in map {
        wtr.write_record(&[k.to_string(), v.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

fn print_elapsed_time(d: Duration) {
    println!(
        "Elapsed time: {}m {}s",
        d.num_minutes(),
        d.num_milliseconds() as f64 / 1000.
    );
}
