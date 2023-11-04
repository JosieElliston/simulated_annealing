use rand::prelude::*;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

use crate::color::*;

const SIZE: usize = 70;
const LOCAL_RADIUS: i32 = 4;

pub(crate) struct Grid {
    neighbor_deltas: Vec<((i32, i32), f32)>,
    colors: Vec<Vec<Color>>,
}

impl Grid {
    pub(crate) fn new_rand(rng: &mut impl Rng) -> Grid {
        Grid {
            neighbor_deltas: (-LOCAL_RADIUS..=LOCAL_RADIUS)
                .flat_map(|y| {
                    (-LOCAL_RADIUS..=LOCAL_RADIUS)
                        .filter_map(|x| {
                            let distance = Grid::point_distance(0, 0, x, y);
                            if distance <= LOCAL_RADIUS as f32 {
                                Some(((x, y), distance))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<((i32, i32), f32)>>()
                        .into_iter()
                })
                .collect(),

            colors: (0..=255)
                .map(|_| (0..=255).map(|_| Color::new_rand(rng)).collect())
                .collect(),
        }
    }

    fn new_better(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        // returns whether after the swap it is better
        let neighbors1: Vec<((usize, usize), f32)> = self
            .neighbor_deltas
            .iter()
            .filter_map(|&((dx, dy), distance)| {
                let x = x1 as i32 + dx;
                let y = y1 as i32 + dy;
                // assert!(Grid::point_distance(x1 as i32, y1 as i32, x, y) == distance);
                if 0 <= x && x < SIZE as i32 && 0 <= y && y < SIZE as i32 {
                    Some(((x as usize, y as usize), distance))
                } else {
                    None
                }
            })
            .collect();
        let neighbors2: Vec<((usize, usize), f32)> = self
            .neighbor_deltas
            .iter()
            .filter_map(|&((dx, dy), distance)| {
                let x = x2 as i32 + dx;
                let y = y2 as i32 + dy;
                // assert!(Grid::point_distance(x2 as i32, y2 as i32, x, y) == distance);
                if 0 <= x && x < SIZE as i32 && 0 <= y && y < SIZE as i32 {
                    Some(((x as usize, y as usize), distance))
                } else {
                    None
                }
            })
            .collect();

        let mut cost0 = 0.0;
        for &((x, y), distance) in neighbors1.iter() {
            cost0 += Grid::cost(
                Color::distance(&self.colors[y1][x1], &self.colors[y][x]),
                distance,
            );
        }
        for &((x, y), distance) in neighbors2.iter() {
            cost0 += Grid::cost(
                Color::distance(&self.colors[y2][x2], &self.colors[y][x]),
                distance,
            );
        }

        (self.colors[y1][x1], self.colors[y2][x2]) = (self.colors[y2][x2], self.colors[y1][x1]);

        let mut cost1 = 0.0;
        for &((x, y), distance) in neighbors1.iter() {
            cost1 += Grid::cost(
                Color::distance(&self.colors[y1][x1], &self.colors[y][x]),
                distance,
            );
            if cost1 > cost0 {
                // old is better
                return false;
            }
        }
        for &((x, y), distance) in neighbors2.iter() {
            cost1 += Grid::cost(
                Color::distance(&self.colors[y2][x2], &self.colors[y][x]),
                distance,
            );
            if cost1 > cost0 {
                // old is better
                return false;
            }
        }

        debug_assert!(cost1 <= cost0);
        true
    }

    pub(crate) fn tick(&mut self, rng: &mut impl Rng, temperature: f32) -> bool {
        // returns whether pixels were swapped

        let x1 = rng.gen_range(0..SIZE);
        let y1 = rng.gen_range(0..SIZE);
        let x2 = rng.gen_range(0..SIZE);
        let y2 = rng.gen_range(0..SIZE);
        if x1 == x2 && y1 == y2 {
            return false;
        }
        // if (x1, y1) in fixed_pixels or (x2, y2) in fixed_pixels:
        //     return False

        let new_better = self.new_better(x1, y1, x2, y2);
        if new_better || rng.gen_range(0.0..1.0) < temperature {
            // keep them swapped
            true
        } else {
            // revert the swap
            (self.colors[y1][x1], self.colors[y2][x2]) = (self.colors[y2][x2], self.colors[y1][x1]);
            // self.approx_cost[y1][x1] = cost1;
            // self.approx_cost[y2][x2] = cost2;
            false
        }
    }

    fn point_distance(x0: i32, y0: i32, x1: i32, y1: i32) -> f32 {
        (((x0 - x1).pow(2) + (y0 - y1).pow(2)) as f32).sqrt()
        // ((x0 as f32 - x1 as f32).powi(2) + (y0 as f32 - y1 as f32).powi(2)).sqrt()
    }

    fn cost(color_dist: f32, point_dist: f32) -> f32 {
        //  if x0 == x1 and y0 == y1:
        //      return 0
        //  pd = point_distance(x0, y0, x1, y1)
        // assert!(point_dist <= LOCAL_RADIUS as f32);

        //  constant
        //  return color_dist

        //  linear drop off
        (LOCAL_RADIUS as f32 - point_dist + 1.0) * color_dist

        //  linear increase
        //  return point_dist*color_dist

        //  constant
        //  if point_dist < .75*local_radius:
        //      return color_dist
        //  else:
        //      return -color_dist/2

        //  linear drop off to increase
        //  if point_dist < local_radius/2:
        //      return (local_radius-point_dist)*color_dist
        //  else:
        //      return -point_dist*color_dist/2
    }

    // fn pixel_cost(&self, x0: i32, y0: i32) -> f32 {
    //     let mut cost = 0.0;
    //     for &((dx, dy), distance) in self.neighbor_deltas.iter() {
    //         let x = x0 + dx;
    //         let y = y0 + dy;
    //         if !(0 <= x && x < SIZE as i32 && 0 <= y && y < SIZE as i32) {
    //             continue;
    //         }
    //         assert!(Grid::point_distance(x0, y0, x, y) == distance);
    //         // cost += Grid::cost(
    //         //     Color::distance(self.colors[y0][x0], self.colors[y0][x]),
    //         //     distance
    //         // ); // TODO: are these indexes wrong
    //         cost += Grid::cost(
    //             Color::distance(
    //                 &self.colors[y0 as usize][x0 as usize],
    //                 &self.colors[y as usize][x as usize],
    //             ),
    //             distance,
    //         );
    //     }
    //     cost
    // }

    // fn total_cost(&self) -> f32 {
    //     (0..SIZE)
    //         .map(|y| {
    //             (0..SIZE)
    //                 .map(|x| self.pixel_cost(x as i32, y as i32))
    //                 .sum::<f32>()
    //         })
    //         .sum::<f32>()
    //     // sum(sum(self.pixel_cost(i, j) for j in (0..SIZE as i32)) for i in 0..SIZE as i32)
    // }

    pub(crate) fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        for (y, line) in self.colors.iter().enumerate() {
            for (x, &color) in line.iter().enumerate() {
                draw_handle.draw_rectangle(
                    (x * crate::SCREEN_SIZE / SIZE) as i32,
                    (y * crate::SCREEN_SIZE / SIZE) as i32,
                    (crate::SCREEN_SIZE / SIZE) as i32,
                    (crate::SCREEN_SIZE / SIZE) as i32,
                    Into::<raylib::prelude::Color>::into(color),
                );
                // draw_handle.draw_pixel(
                //     x as i32,
                //     SIZE as i32 - y as i32 - 1,
                //     Into::<raylib::prelude::Color>::into(color),
                // );
            }
        }
    }
}
