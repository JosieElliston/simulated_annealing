use ::core::time;

use grid::Grid;
use rand::prelude::*;
use raylib::prelude::*;

mod color;
mod grid;

pub(crate) const SCREEN_SIZE: usize = 700;
const TEMPERATURE_0: f32 = 0.05;
// const STEP_MAX: usize = 50000;
const STEP_MAX: usize = 5000000;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut grid = Grid::new_rand(&mut rng);
    test_time(&mut rng, &mut grid);
    // render(&mut rng, &mut grid);
    // raylib_render(&mut rng, &mut grid);
}

fn test_time(rng: &mut impl Rng, grid: &mut Grid) {
    let steps = 100000;
    let start = std::time::Instant::now();
    for _ in 0..steps {
        grid.tick(rng, 0.0);
    }
    let time = start.elapsed();
    println!("time: {}", time.as_secs_f32());
    println!("nanos per step: {}", time.as_nanos() as f32 / steps as f32);
}

fn calc_temp(cur_step: usize) -> f32 {
    (0.9999_f32).powi(cur_step as i32)
    // (-(cur_step as f32) / 100000.0).exp().min(TEMPERATURE_0)
    // f32::max(
    //     0.0,
    //     TEMPERATURE_0 * (1.0 - cur_step as f32 / STEP_MAX as f32),
    // )
}

fn render(rng: &mut impl Rng, grid: &mut Grid) {
    let mut cur_step: usize = 0;
    let mut cur_frame: usize = 0;
    loop {
        cur_frame += 1;

        let start = std::time::Instant::now();
        while start.elapsed().as_secs_f32() < 0.01 {
            if grid.tick(rng, calc_temp(cur_step)) {
                cur_step += 1;
            }
        }
        if cur_frame % 1000 == 0 {
            println!("saved image");
            grid.save_image();
        }
    }
}

fn raylib_render(rng: &mut impl Rng, grid: &mut Grid) {
    let mut cur_step: usize = 0;
    let mut cur_frame: usize = 0;

    let (mut rl, thread) = raylib::init()
        .size((SCREEN_SIZE) as i32, SCREEN_SIZE as i32)
        .title("simulated annealing")
        .build();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        cur_frame += 1;

        let start = std::time::Instant::now();
        while start.elapsed().as_secs_f32() < 0.01 {
            if grid.tick(rng, calc_temp(cur_step)) {
                cur_step += 1;
            }
        }
        if cur_frame % 100 == 0 {
            println!("saved image");
            grid.save_image();
        }

        // println!("cur_step: {}", cur_step);
        // println!("temperature: {}", calc_temp(cur_step));
        let mut draw_handle: RaylibDrawHandle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        grid.draw(&mut draw_handle);
    }
}
