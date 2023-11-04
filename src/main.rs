use ::core::time;

use grid::Grid;
use rand::prelude::*;
use raylib::prelude::*;

mod color;
mod grid;

pub(crate) const SCREEN_SIZE: usize = 700;
const TEMPERATURE_0: f32 = 0.1;
const STEP_MAX: usize = 50000;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut rng = thread_rng();
    let mut grid = Grid::new_rand(&mut rng);
    test_time(&mut rng, &mut grid);
}

fn test_time(rng: &mut impl Rng, grid: &mut Grid) {
    let steps = 1000000;
    let start = std::time::Instant::now();
    for _ in 0..steps {
        grid.tick(rng, 0.0);
    }
    let time = start.elapsed();
    println!("time: {}", time.as_secs_f32());
    println!("nanos per step: {}", time.as_nanos() as f32 / steps as f32);
}

fn render(rng: &mut impl Rng, grid: &mut Grid) {
    let mut cur_step: usize = 0;

    let (mut rl, thread) = raylib::init()
        .size((SCREEN_SIZE) as i32, SCREEN_SIZE as i32)
        .title("voxel renderer")
        .build();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let start = std::time::Instant::now();
        while start.elapsed().as_secs_f32() < 0.01 {
            let temperature = f32::max(
                0.0,
                TEMPERATURE_0 * (1.0 - cur_step as f32 / STEP_MAX as f32),
            );
            grid.tick(rng, temperature);
            cur_step += 1;
        }

        let mut draw_handle: RaylibDrawHandle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        grid.draw(&mut draw_handle);
    }
}
