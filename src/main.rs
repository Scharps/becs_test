use std::time::{Duration, Instant};

use becs::World;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1200;
const HEIGHT: usize = 360;

struct Position(f32, f32);
struct Speed(f32);

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test ECS", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut world = World::new();
    let entity = world.new_entity();
    world.add_component_to_entity(entity, Position(50.0, 100.0));
    world.add_component_to_entity(entity, Speed(200.0));

    let width = 20;
    let height = 40;

    let mut timing = Timing::new();

    let (mut w, mut a, mut s, mut d, mut shift) = (false, false, false, false, false);
    while window.is_open() {
        let mut positions = world.borrow_component_vec_mut::<Position>().unwrap();
        let speeds = world.borrow_component_vec::<Speed>().unwrap();

        // Capture input
        capture_input(&mut w, &window, &mut a, &mut s, &mut d, &mut shift);

        // Draw to buffer
        for position in positions.iter().filter_map(|position| position.as_ref()) {
            draw_as_rect(&mut buffer, position, width, height);
        }

        // Display buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        let delta = timing.delta().as_secs_f32();
        // Update position
        let zip = positions.iter_mut().zip(speeds.iter());
        for (position, speed) in
            zip.filter_map(|(position, speed)| Some((position.as_mut()?, speed.as_ref()?)))
        {
            let (mut x, mut y) = (0.0, 0.0);
            if w {
                y += -1.0 * speed.0 * delta;
            }
            if a {
                x += -1.0 * speed.0 * delta;
            }
            if s {
                y += 1.0 * speed.0 * delta;
            }
            if d {
                x += 1.0 * speed.0 * delta;
            }

            position.0 += x;
            position.1 += y;
        }

        // Clear
        clear_buffer(&mut buffer);
    }
}

fn capture_input(
    w: &mut bool,
    window: &Window,
    a: &mut bool,
    s: &mut bool,
    d: &mut bool,
    shift: &mut bool,
) {
    *w = window.is_key_down(Key::W);
    *a = window.is_key_down(Key::A);
    *s = window.is_key_down(Key::S);
    *d = window.is_key_down(Key::D);
    *shift = window.is_key_down(Key::LeftShift);
}

fn clear_buffer(buffer: &mut Vec<u32>) {
    for p in buffer.iter_mut() {
        *p = 0;
    }
}

fn draw_as_rect(buffer: &mut [u32], position: &Position, width: usize, height: usize) {
    let (x, y) = (position.0 as isize, position.1 as isize);
    for j in y..=y + height as isize {
        for i in x..=x + width as isize {
            let index = j * WIDTH as isize + i;
            if i < WIDTH as isize && j < HEIGHT as isize && i > -1 && j > -1 {
                buffer[index as usize] = 0xFFFFFF;
            }
        }
    }
}

struct Timing {
    last_reset: Instant,
}

impl Timing {
    fn new() -> Self {
        Self {
            last_reset: Instant::now(),
        }
    }

    fn delta(&mut self) -> Duration {
        let delta = self.last_reset.elapsed();
        self.last_reset = Instant::now();
        delta
    }
}
