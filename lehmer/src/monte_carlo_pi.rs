use crate::core::FastU32;
use plotters::prelude::*;
use rand::Fill;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use rand_pcg::Pcg32;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use statrs::function::gamma::gamma;
use std::f64;
use std::f64::consts::PI;
use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdFloat;
use std::simd::LaneCount;
use std::simd::Simd;
use std::simd::SupportedLaneCount;

const PRECISION: f64 = 0.00001;
const MAX_ITERATIONS: u64 = 2_500_000_000;

fn is_precision_reached(estimate: f64) -> bool {
    f64::abs(estimate - PI) < PRECISION
}

fn estimate(count: u32, iterations: u32) -> f64 {
    (count as f64) * 4.0 / (iterations as f64)
}

// area of n-sphere / area of n-cube -> ratio of inside/outside of n-sphere
// check whether generated point is outside of unit sphere using euclidian norm
// (root canceled out)
fn estimate_n(count: u64, iterations: u64, n: usize) -> f64 {
    let nominator: f64 = count as f64 * gamma(n as f64 / 2.0 + 1.0) * u128::pow(2, n as u32) as f64;
    let demoniator: f64 = iterations as f64;
    f64::powf(nominator / demoniator, 2.0 / n as f64)
}

pub fn estimate_pi<R: Rng + SeedableRng>(seed: u64) -> u32 {
    let mut rng = R::seed_from_u64(seed);
    let mut iterations = 0;
    let mut count = 0;

    while !is_precision_reached(estimate(count, iterations)) {
        let x: f32 = rng.gen_range(0.0..1.0);
        let y: f32 = rng.gen_range(0.0..1.0);
        let p: f32 = x * x + y * y;

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }
    iterations
}

pub fn estimate_pi_n<R: Rng + SeedableRng>(n: usize, seed: u64) -> u64 {
    let mut rng = R::seed_from_u64(seed);
    let mut iterations: u64 = 0;
    let mut count: u64 = 0;

    let mut rns = vec![0.0; n];
    while !is_precision_reached(estimate_n(count, iterations, n)) && iterations < MAX_ITERATIONS {
        for i in 0..n {
            rns[i] = rng.gen_range(0.0..1.0);
        }
        let p: f32 = rns.iter().map(|rn| rn * rn).sum();

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }
    if iterations == MAX_ITERATIONS {
        println!("Seed {:?} stuck", seed);
    }
    iterations
}

pub fn estimate_pi_simd<R: Rng + SeedableRng>(seed: u64) -> u32 {
    let mut rng = R::seed_from_u64(seed);
    let mut iterations = 0;
    let mut count = 0;

    let mut x_buff: [f32; 64] = [0.0; 64];
    let mut y_buff: [f32; 64] = [0.0; 64];

    let splat = Simd::splat(1.0);

    while !is_precision_reached(estimate(count, iterations)) {
        Fill::try_fill(&mut x_buff, &mut rng).unwrap();
        Fill::try_fill(&mut y_buff, &mut rng).unwrap();

        let x = Simd::from(x_buff);
        let y = Simd::from(y_buff);
        let p = x * x + y * y;

        iterations += 64;
        count += p.simd_lt(splat).to_bitmask().count_ones();
    }

    iterations
}

pub fn estimate_pi_n_simd<R, const N: usize>(seed: u64) -> u64
where
    R: Rng + SeedableRng,
    LaneCount<N>: SupportedLaneCount,
{
    let mut rng = R::seed_from_u64(seed);
    let mut iterations = 0;
    let mut count = 0;

    let mut rns: [f32; N] = [0.0; N];

    while !is_precision_reached(estimate_n(count, iterations, N)) && iterations < MAX_ITERATIONS {
        Fill::try_fill(&mut rns[0..N], &mut rng).unwrap();

        let xs = Simd::from(rns);
        let p = (xs * xs).reduce_sum();

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }

    iterations
}

const ITERATIONS: usize = 50;
const DIMENSIONS: usize = 16;
const STARTING_DIMENSION: usize = 2;

pub fn check_difference_print() {
    fn mean_iterations<R>(buf: &[u64], dimension: usize) -> f64
    where
        R: Rng + SeedableRng,
    {
        buf.par_iter()
            .enumerate()
            .map(|(i, _)| -> u64 { estimate_pi_n::<R>(dimension, i as u64 + 1) })
            .reduce(|| 0, |a, b| a + b) as f64
            / ITERATIONS as f64
    }
    let buf: [u64; ITERATIONS] = [0; ITERATIONS];

    let mut result_lehmer: [f64; DIMENSIONS - STARTING_DIMENSION + 1] =
        [0.0; DIMENSIONS - STARTING_DIMENSION + 1];
    let mut result_pcg: [f64; DIMENSIONS - STARTING_DIMENSION + 1] =
        [0.0; DIMENSIONS - STARTING_DIMENSION + 1];

    for dimension in STARTING_DIMENSION..=DIMENSIONS {
        println!("dimension: {:?}", dimension);

        println!("lehmer");
        result_lehmer[dimension - STARTING_DIMENSION] = mean_iterations::<FastU32>(&buf, dimension);
        println!("pcg");
        result_pcg[dimension - STARTING_DIMENSION] = mean_iterations::<Pcg32>(&buf, dimension);
    }
    println!("{:?}", result_lehmer);
    println!("{:?}", result_pcg);
}

#[derive(Copy, Clone, Debug)]
struct Iterations {
    mean_iterations: u32,
    dimension: usize,
}

impl Iterations {
    fn new(mean_iterations: u32, dimension: usize) -> Iterations {
        Iterations {
            mean_iterations,
            dimension,
        }
    }
}

pub fn check_difference(path: &str, different_seeds: u32, dimensions: usize) {
    fn mean_iterations<R>(buf: &[u32], different_seeds: u32, dimension: usize) -> f64
    where
        R: Rng + SeedableRng,
    {
        buf.par_iter()
            .enumerate()
            .map(|(i, _)| -> u64 { estimate_pi_n::<R>(dimension, i as u64 + 1) as u64 })
            .reduce(|| 0, |a, b| a + b) as f64
            / different_seeds as f64
    }

    let buf: Vec<u32> = vec![0; different_seeds as usize];

    let mut result_lehmer = Vec::new();
    let mut result_pcg = Vec::new();
    let mut result_cha = Vec::new();

    for dimension in STARTING_DIMENSION..=dimensions {
        println!("dimension: {:?}", dimension);
        println!("lehmer");
        result_lehmer.push(Iterations::new(
            mean_iterations::<FastU32>(&buf, different_seeds, dimension) as u32,
            dimension,
        ));

        println!("pcg");
        result_pcg.push(Iterations::new(
            mean_iterations::<Pcg32>(&buf, different_seeds, dimension) as u32,
            dimension,
        ));

        println!("chacha20");
        result_cha.push(Iterations::new(
            mean_iterations::<ChaCha20Rng>(&buf, different_seeds, dimension) as u32,
            dimension,
        ));
    }
    plot_mean_iterations(
        path,
        &result_lehmer,
        &result_pcg,
        &result_cha,
        different_seeds,
        dimensions,
    )
    .unwrap();
}

#[derive(Copy, Clone, Debug)]
struct Estimate {
    estimate: f64,
    iteration: usize,
}

pub fn estimate_fixed_iterations<R: Rng + SeedableRng>(
    path: &str,
    n: usize,
    seed: u64,
    iterations: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = R::seed_from_u64(seed);
    let mut counts: Vec<(u64, usize)> = Vec::new();

    let mut current_count = 0;
    for i in 0..iterations {
        let mut rns = Vec::new();
        for _ in 0..n {
            rns.push(rng.gen_range(0.0..1.0));
        }
        let p: f32 = rns.iter().map(|rn| rn * rn).sum();
        if p < 1.0 {
            current_count += 1;
        }
        counts.push((current_count, i));
    }

    let estimates: Vec<Estimate> = counts
        .par_iter()
        .map(|(count, no_iter)| Estimate {
            estimate: estimate_n(*count, *no_iter as u64, n),
            iteration: *no_iter,
        })
        .collect();

    plot_fixed_iterations(path, &estimates, n, iterations)
}

fn plot_mean_iterations(
    path: &str,
    lehmer: &[Iterations],
    pcg32: &[Iterations],
    chacha20: &[Iterations],
    different_seeds: u32,
    dimensions: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Mean Iterations for Dimension for Fixed Precision",
            ("sans-serif", (5).percent_height()),
        )
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (2u32..(dimensions as u32 + 1)).with_key_points((2..=(dimensions as u32)).collect()),
            (10_000u32..130_000_000u32).log_scale(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Dimension")
        .y_desc(format!(
            "Mean Iterations for {different_seeds} Different Seeds"
        ))
        .draw()?;

    let color = Palette99::pick(0).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            lehmer.iter().map(
                |&Iterations {
                     mean_iterations,
                     dimension,
                     ..
                 }| (dimension as u32, mean_iterations as u32),
            ),
            color.stroke_width(2),
        ))?
        .label("Lehmer")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    let color = Palette99::pick(1).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            pcg32.iter().map(
                |&Iterations {
                     mean_iterations,
                     dimension,
                     ..
                 }| (dimension as u32, mean_iterations as u32),
            ),
            color.stroke_width(2),
        ))?
        .label("PCG32")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    let color = Palette99::pick(2).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            chacha20.iter().map(
                |&Iterations {
                     mean_iterations,
                     dimension,
                     ..
                 }| (dimension as u32, mean_iterations as u32),
            ),
            color.stroke_width(2),
        ))?
        .label("ChaCha20")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    chart.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", path);

    Ok(())
}

fn plot_fixed_iterations(
    path: &str,
    estimates: &[Estimate],
    n: usize,
    iterations: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Estimate Time Series for Fixed Iterations for Precision {PRECISION} and Dimension {n}"),
            ("sans-serif", (5).percent_height()),
        )
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            0u32..(iterations as u32),
            280_000u32..((3.4 as f64 / PRECISION) as u32),
        )?;

    chart
        .configure_mesh()
        .x_desc("Iteration")
        .y_desc("Estimate / Precision")
        .draw()?;

    let color = Palette99::pick(0).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            estimates.iter().map(
                |&Estimate {
                     estimate,
                     iteration,
                     ..
                 }| (iteration as u32, (estimate / PRECISION) as u32),
            ),
            color.stroke_width(1),
        ))?
        .label("Estimate")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    let color = Palette99::pick(1).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            (0..iterations).map(|i| (i as u32, (PI / PRECISION) as u32)),
            color.stroke_width(1),
        ))?
        .label("Pi")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    chart.configure_series_labels().border_style(BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", path);

    Ok(())
}
