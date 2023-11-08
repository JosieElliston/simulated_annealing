use grid::Grid;
use rand::prelude::*;
use raylib::prelude::*;

mod color;
mod grid;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    // let mut rng = rand::rngs::SmallRng::from_entropy();
    // test_throughput();
    // test_time_hist(&mut rng);
    // render(&mut rng);
    raylib_render();
}

// did 100000 ticks in 0.19389904 secs
// 1938.9904 nanos per tick
// did 37666 steps in 0.19389904 secs
// 5147.853 nanos per step

// did 20 ticks in 3.7371955 secs
// 186859780 nanos per tick
// did 2800954 steps in 3.7371955 secs
// 1334.258 nanos per step

fn test_throughput() {
    const TEST_TICKS: u128 = 20;
    // const TICKS: usize = 1;
    let thread_count: usize = rayon::max_num_threads();
    // const TEST_TICKS: u128 = 10000;
    // let thread_count: usize = 8;

    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut rngs: Vec<SmallRng> = (0..thread_count)
        .map(|_| SmallRng::from_rng(&mut rng).unwrap())
        .collect();
    let mut grid = Grid::new_rand(&mut rng);
    let mut ticks = 0;
    let mut steps = 0;

    let start = std::time::Instant::now();
    for _ in 0..TEST_TICKS {
        steps += grid.par_tick(&mut rngs, 0.0);
        ticks += 1;
    }
    let time = start.elapsed();
    println!("did {} ticks in {} secs", ticks, time.as_secs_f32());
    println!("{} nanos per tick", time.as_nanos() as f32 / ticks as f32);
    println!("did {} steps in {} secs", steps, time.as_secs_f32());
    println!("{} nanos per step", time.as_nanos() as f32 / steps as f32);
}

// fn tick_n_steps(rng: &mut impl Rng, grid: &mut Grid, steps: usize) {
//     for _ in 0..steps {
//         // std::hint::black_box(grid.tick(rng, 0.0));
//         grid.par_tick(rng, 1, 0.0);
//     }
// }

// fn test_time_hist(rng: &mut impl Rng) {
//     const TEST_N: usize = 100;
//     const TEST_STEPS: u128 = 1000;
//     let mut hist = histo::Histogram::with_buckets(200);
//     for test_n in 0..TEST_N {
//         let mut grid = Grid::new_rand(rng);
//         let start = std::time::Instant::now();
//         std::hint::black_box(tick_n_steps(rng, &mut grid, TEST_STEPS as usize));
//         let time = start.elapsed();
//         println!("time: {}", time.as_secs_f32());
//         let nanos_per_step = time.as_nanos() as f32 / TEST_STEPS as f32;
//         println!("nanos per step: {}", nanos_per_step);
//         // warm up so the first few aren't weirdly slow
//         if test_n > 8 {
//             hist.add(nanos_per_step as u64);
//         }
//     }
//     println!("{}", hist);
// }

fn calc_temp(cur_step: usize) -> f32 {
    // exponential drop off
    (0.99999_f32).powi(cur_step as i32) / 2.0

    // const TEMPERATURE_0: f32 = 0.05;
    // const STEP_MAX: usize = 50000;
    // const STEP_MAX: usize = 5000000;
    // linear drop off
    // (-(cur_step as f32) / 100000.0).exp().min(TEMPERATURE_0)
    // f32::max(
    //     0.0,
    //     TEMPERATURE_0 * (1.0 - cur_step as f32 / STEP_MAX as f32),
    // )
}

// fn render(rng: &mut impl Rng) {
//     let mut grid = Grid::new_rand(rng);
//     let mut cur_step: usize = 0;
//     let mut cur_frame: usize = 0;
//     loop {
//         cur_frame += 1;

//         let start = std::time::Instant::now();
//         while start.elapsed().as_secs_f32() < 0.01 {
//             cur_step += grid.par_tick(rng, 1, calc_temp(cur_step));
//         }
//         if cur_frame % 1000 == 0 {
//             println!("saved image");
//             grid.save_image();
//         }
//     }
// }

fn raylib_render() {
    const SCREEN_SIZE: usize = 700;
    let thread_count: usize = rayon::max_num_threads();

    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut rngs: Vec<SmallRng> = (0..thread_count)
        .map(|_| SmallRng::from_rng(&mut rng).unwrap())
        .collect();
    let mut grid = Grid::new_rand(&mut rng);
    // grid.sort();
    let mut cur_step: usize = 0;
    let mut last_saved = std::time::Instant::now();

    let (mut rl, thread) = raylib::init()
        .size((SCREEN_SIZE) as i32, SCREEN_SIZE as i32)
        .title("simulated annealing")
        .build();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let start = std::time::Instant::now();
        while start.elapsed().as_secs_f32() < 0.01 {
            cur_step += grid.par_tick(&mut rngs, calc_temp(cur_step));
        }

        if last_saved.elapsed().as_secs_f32() > 1.0 {
            println!("saved image");
            grid.save_image();
            last_saved = std::time::Instant::now();
        }

        println!("cur_step: {}", cur_step);
        println!("temperature: {}", calc_temp(cur_step));
        let mut draw_handle: RaylibDrawHandle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        grid.draw(&mut draw_handle);
    }
}
