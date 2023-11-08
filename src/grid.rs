use rand::prelude::*;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};
use rayon::prelude::*;

use crate::color::*;

const SIZE: usize = 256; // what ive been benching
// const SIZE: usize = 1024*2;

const LOCAL_RADIUS: i32 = (SIZE / 16) as i32;
const STEP_RADIUS: i32 = LOCAL_RADIUS * 2;
// baseline: 3363.0659340659336
// new but incorrect: 2052.802197802198

pub(crate) struct Grid {
    neighbor_deltas: Vec<((i32, i32), f32)>,
    // neighbors: Vec<Vec<Vec<((usize, usize), f32)>>>,
    colors: Vec<Vec<Color>>,
}

// TODO: remove huge neighbors vec, because of wrapping its branchless now, just need to do (x + SIZE + neighbor_deltas) % SIZE
impl Grid {
    pub(crate) fn new_rand(rng: &mut impl Rng) -> Grid {
        let neighbor_deltas: Vec<((i32, i32), f32)> = (-LOCAL_RADIUS..=LOCAL_RADIUS)
            .flat_map(|y| {
                (-LOCAL_RADIUS..=LOCAL_RADIUS)
                    .filter_map(|x| {
                        if x == 0 && y == 0 {
                            None
                        } else {
                            let distance = Grid::point_distance(0, 0, x, y);
                            if distance <= LOCAL_RADIUS as f32 {
                                Some(((x, y), distance))
                            } else {
                                None
                            }
                        }
                    })
                    .collect::<Vec<((i32, i32), f32)>>()
                    .into_iter()
            })
            .collect();
        // let neighbors: Vec<Vec<Vec<((usize, usize), f32)>>> = (0..SIZE)
        // .map(|y0| {
        //     (0..SIZE)
        //         .map(|x0| {
        //             neighbor_deltas
        //                 .iter()
        //                 .map(|&((dx, dy), distance)| {
        //                     let mut x = x0 as i32 + dx;
        //                     let mut y = y0 as i32 + dy;
        //                     if x < 0 {
        //                         x += SIZE as i32;
        //                     } else if x >= SIZE as i32 {
        //                         x -= SIZE as i32;
        //                     }
        //                     assert!(0 <= x && x < SIZE as i32);
        //                     if y < 0 {
        //                         y += SIZE as i32;
        //                     } else if y >= SIZE as i32 {
        //                         y -= SIZE as i32;
        //                     }
        //                     assert!(0 <= y && y < SIZE as i32);
        //                     ((x as usize, y as usize), distance)
        //                 })
        //                 .collect()
        //         })
        //         .collect()
        // })
        // .collect();
        Grid {
            neighbor_deltas,
            colors: (0..SIZE)
                .map(|_| (0..SIZE).map(|_| Color::new_rand(rng)).collect())
                .collect(),
        }
    }

    // fn sort_helper(colors: &mut Vec<Color>) {
    //     for i in 0..colors.len() {
    //         for j in i + 1..colors.len() {
    //             if colors[i].b > colors[j].b {
    //                 (colors[i], colors[j]) = (colors[j], colors[i]);
    //             }
    //         }
    //     }
    // }

    // pub(crate) fn sort(&mut self) {
    //     for i in 0..SIZE * SIZE {
    //         for j in i + 1..SIZE * SIZE {
    //             if self.colors[i / SIZE][i % SIZE].b > self.colors[j / SIZE][j % SIZE].b {
    //                 (
    //                     self.colors[i / SIZE][i % SIZE],
    //                     self.colors[j / SIZE][j % SIZE],
    //                 ) = (
    //                     self.colors[j / SIZE][j % SIZE],
    //                     self.colors[i / SIZE][i % SIZE],
    //                 );
    //             }
    //         }
    //     }
    // }

    #[inline(always)]
    fn modulo(&self, i: usize, i_delta: i32) -> usize {
        (((i + SIZE) as i32) + i_delta) as usize % SIZE
        // let i = (((i + SIZE) as i32) + i_delta) as usize;
        // if i >= 2 * SIZE {
        //     i - 2 * SIZE
        // } else if i >= SIZE {
        //     i - SIZE
        // } else {
        //     i
        // }
        // ((((i + SIZE) as i32) + i_delta) % SIZE as i32) as usize
    }

    #[inline(always)]
    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        (self.colors[y1][x1], self.colors[y2][x2]) = (self.colors[y2][x2], self.colors[y1][x1]);
    }

    // TODO: make it so it doesn't need to actually do the swap
    // fn new_better_oracle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) -> (f32, f32) {
    //     // returns whether after the swap it is better
    //     let mut cost0 = 0.0;
    //     for &((dx, dy), distance) in self.neighbor_deltas.iter() {
    //         let x = self.modulo(x1, dx);
    //         let y = self.modulo(y1, dy);
    //         cost0 += Grid::cost(
    //             Color::distance(&self.colors[y1][x1], &self.colors[y][x]),
    //             distance,
    //         );
    //     }
    //     for &((dx, dy), distance) in self.neighbor_deltas.iter() {
    //         let x = self.modulo(x2, dx);
    //         let y = self.modulo(y2, dy);
    //         cost0 += Grid::cost(
    //             Color::distance(&self.colors[y2][x2], &self.colors[y][x]),
    //             distance,
    //         );
    //     }

    //     self.swap(x1, y1, x2, y2);

    //     let mut cost1 = 0.0;
    //     for &((dx, dy), distance) in self.neighbor_deltas.iter() {
    //         let x = self.modulo(x1, dx);
    //         let y = self.modulo(y1, dy);
    //         cost1 += Grid::cost(
    //             Color::distance(&self.colors[y1][x1], &self.colors[y][x]),
    //             distance,
    //         );
    //         // if cost1 > cost0 {
    //         //     // old is better
    //         //     return false;
    //         // }
    //     }
    //     for &((dx, dy), distance) in self.neighbor_deltas.iter() {
    //         let x = self.modulo(x2, dx);
    //         let y = self.modulo(y2, dy);
    //         cost1 += Grid::cost(
    //             Color::distance(&self.colors[y2][x2], &self.colors[y][x]),
    //             distance,
    //         );
    //         // if cost1 > cost0 {
    //         //     // old is better
    //         //     return false;
    //         // }
    //     }

    //     self.swap(x1, y1, x2, y2);

    //     (cost0, cost1)
    // }

    // slow
    // fn pseudo_swap_get(
    //     &self,
    //     x1: usize,
    //     y1: usize,
    //     x2: usize,
    //     y2: usize,
    //     x: usize,
    //     y: usize,
    // ) -> &Color {
    // gets the color as though p1 and p2 were swapped
    //     if x == x1 && y == y1 {
    //         &self.colors[y2][x2]
    //     } else if x == x2 && y == y2 {
    //         &self.colors[y1][x1]
    //     } else {
    //         &self.colors[y][x]
    //     }
    // }

    fn new_better(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> (f32, f32) {
        // returns cost without swap and cost with cost
        // not actually correct because pseudo_swap_get is slow
        let mut original_cost = 0.0;
        let mut swapped_cost = 0.0;
        self.neighbor_deltas
            .iter()
            .for_each(|&((dx, dy), distance)| {
                let x = self.modulo(x1, dx);
                let y = self.modulo(y1, dy);
                original_cost += Grid::cost(
                    Color::distance(&self.colors[y1][x1], &self.colors[y][x]),
                    distance,
                );
                swapped_cost += Grid::cost(
                    Color::distance(&self.colors[y2][x2], &self.colors[y][x]),
                    // Color::distance(
                    //     &self.colors[y2][x2],
                    //     self.pseudo_swap_get(x1, y1, x2, y2, x, y),
                    // ),
                    distance,
                );
            });
        self.neighbor_deltas
            .iter()
            .for_each(|&((dx, dy), distance)| {
                let x = self.modulo(x2, dx);
                let y = self.modulo(y2, dy);
                original_cost += Grid::cost(
                    Color::distance(&self.colors[y2][x2], &self.colors[y][x]),
                    distance,
                );
                swapped_cost += Grid::cost(
                    Color::distance(&self.colors[y1][x1], &self.colors[y][x]),
                    // Color::distance(
                    //     &self.colors[y1][x1],
                    //     self.pseudo_swap_get(x1, y1, x2, y2, x, y),
                    // ),
                    distance,
                );
            });

        (original_cost, swapped_cost)
    }

    #[inline(always)]
    fn accept_prob(cost0: f32, cost1: f32, temperature: f32) -> f32 {
        if cost1 < cost0 {
            1.0
        } else {
            let cost_ratio = cost0 / cost1;
            // println!("cost ratio: {}", cost_ratio);
            temperature * cost_ratio
        }
    }

    // fn random_adjacent(rng: &mut impl Rng, x1: usize, y1: usize) -> (usize, usize) {
    //     let x2 = {
    //         if rng.gen_bool(0.5) {
    //             if x1 == SIZE - 1 {
    //                 return false;
    //             }
    //             x1 + 1
    //         } else {
    //             if x1 == 0 {
    //                 return false;
    //             }
    //             x1 - 1
    //         }
    //     };
    //     let y2 = {
    //         if rng.gen_bool(0.5) {
    //             if y1 == SIZE - 1 {
    //                 return false;
    //             }
    //             y1 + 1
    //         } else {
    //             if y1 == 0 {
    //                 return false;
    //             }
    //             y1 - 1
    //         }
    //     };
    //     (x2, y2)
    // }

    fn random_in_local_radius(rng: &mut impl Rng, x1: usize, y1: usize) -> (usize, usize) {
        let x2 = x1 as i32 + rng.gen_range(-STEP_RADIUS..=STEP_RADIUS);
        let x2 = {
            if x2 < 0 {
                x2 + SIZE as i32
            } else if x2 >= SIZE as i32 {
                x2 - SIZE as i32
            } else {
                x2
            }
        } as usize;
        let y2 = y1 as i32 + rng.gen_range(-STEP_RADIUS..=STEP_RADIUS);
        let y2 = {
            if y2 < 0 {
                y2 + SIZE as i32
            } else if y2 >= SIZE as i32 {
                y2 - SIZE as i32
            } else {
                y2
            }
        } as usize;
        (x2, y2)
    }

    fn random_any(rng: &mut impl Rng, x1: usize, y1: usize) -> (usize, usize) {
        let x2 = rng.gen_range(0..SIZE);
        let y2 = rng.gen_range(0..SIZE);
        (x2, y2)
    }

    pub(crate) fn par_tick(
        &mut self,
        rngs: &mut [rand::rngs::SmallRng],
        temperature: f32,
    ) -> usize {
        // returns the number of steps taken
        // where a step is a successful swap

        // ticks needs to be small to preserve stability
        // assert!(ticks < 16);

        // println!("rayon::max_num_threads(): {}", rayon::max_num_threads());
        // instead of (0..rayon::max_num_threads()), use a vec of Rng
        let swap_is: Vec<(usize, usize, usize, usize)> = rngs
            .par_iter_mut()
            .filter_map(|rng: &mut SmallRng| {
                let x1 = rng.gen_range(0..SIZE);
                let y1 = rng.gen_range(0..SIZE);
                // let (x2, y2) = Grid::random_adjacent(rng, x1, y1);
                let (x2, y2) = Grid::random_in_local_radius(rng, x1, y1);
                // let (x2, y2) = Grid::random_any(rng, x1, y1);
                if x1 == x2 && y1 == y2 {
                    return None;
                }

                let (cost0, cost1) = self.new_better(x1, y1, x2, y2);

                if rng.gen_range(0.0..1.0) < Grid::accept_prob(cost0, cost1, temperature) {
                    Some((x1, y1, x2, y2))
                } else {
                    None
                }
            })
            .collect();
        let steps = swap_is.len();
        swap_is.into_iter().for_each(|(x1, y1, x2, y2)| {
            self.swap(x1, y1, x2, y2);
        });
        steps
    }

    // pub(crate) fn tick(&mut self, rng: &mut impl Rng, temperature: f32) -> usize {
    //     // returns the number of steps taken
    //     // where a step is a successful swap

    //     let x1 = rng.gen_range(0..SIZE);
    //     let y1 = rng.gen_range(0..SIZE);
    //     // let (x2, y2) = Grid::random_adjacent(rng, x1, y1);
    //     let (x2, y2) = Grid::random_in_local_radius(rng, x1, y1);
    //     // let (x2, y2) = Grid::random_any(rng, x1, y1);
    //     if x1 == x2 && y1 == y2 {
    //         return 0;
    //     }

    //     let (cost0, cost1) = self.new_better(x1, y1, x2, y2);

    //     if rng.gen_range(0.0..1.0) < Grid::accept_prob(cost0, cost1, temperature) {
    //         self.swap(x1, y1, x2, y2);
    //         1
    //     } else {
    //         0
    //     }
    // }

    #[inline(always)]
    fn point_distance(x0: i32, y0: i32, x1: i32, y1: i32) -> f32 {
        (((x0 - x1).pow(2) + (y0 - y1).pow(2)) as f32).sqrt()
        // ((x0 as f32 - x1 as f32).powi(2) + (y0 as f32 - y1 as f32).powi(2)).sqrt()
    }

    #[inline(always)]
    fn cost(color_dist: f32, point_dist: f32) -> f32 {
        //  if x0 == x1 and y0 == y1:
        //      return 0
        //  pd = point_distance(x0, y0, x1, y1)
        // assert!(point_dist <= LOCAL_RADIUS as f32);

        //  constant
        // color_dist

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
                draw_handle.draw_rectangle_v(
                    raylib::prelude::Vector2 {
                        x: x as f32 * draw_handle.get_screen_width() as f32 / SIZE as f32,
                        y: y as f32 * draw_handle.get_screen_height() as f32 / SIZE as f32,
                    },
                    raylib::prelude::Vector2 {
                        x: draw_handle.get_screen_width() as f32 / SIZE as f32,
                        y: draw_handle.get_screen_height() as f32 / SIZE as f32,
                    },
                    Into::<raylib::prelude::Color>::into(color),
                );
                // draw_handle.draw_rectangle(
                //     (x * crate::SCREEN_SIZE / SIZE) as i32,
                //     (y * crate::SCREEN_SIZE / SIZE) as i32,
                //     (crate::SCREEN_SIZE / SIZE) as i32,
                //     (crate::SCREEN_SIZE / SIZE) as i32,
                //     Into::<raylib::prelude::Color>::into(color),
                // );
                // draw_handle.draw_pixel(
                //     x as i32,
                //     SIZE as i32 - y as i32 - 1,
                //     Into::<raylib::prelude::Color>::into(color),
                // );
            }
        }
    }

    pub(crate) fn save_image(&self) {
        let mut colors: Vec<u8> = vec![0; SIZE * SIZE * 3];
        let mut i = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                let color: raylib::prelude::Color = self.colors[row][col].into();
                colors[i] = color.r;
                colors[i + 1] = color.g;
                colors[i + 2] = color.b;
                i += 3;
            }
        }
        image::save_buffer(
            std::path::Path::new("image.png"),
            &colors,
            SIZE as u32,
            SIZE as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    }
}
