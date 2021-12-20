use becs::World;
use ecs::{configure_ecs, movement_system, Position};
use minifb::{Window, WindowOptions};
use std::{
    sync::mpsc::{channel, Sender},
    thread,
    time::{Duration, Instant},
};

mod ecs;

const WIDTH: usize = 1200;
const HEIGHT: usize = 360;

fn main() {
    let (tx, rx) = channel::<Vec<u32>>();

    thread::spawn(move || {
        let mut window = Window::new("Test ECS", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
        while window.is_open() {
            let buffer = rx.recv().unwrap();
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }
    });

    let mut world = configure_ecs();
    let render_system = render_with_buffer(tx);

    let mut current_time = Instant::now();
    let dt = Duration::from_secs_f64(1.0 / 60.0).as_secs_f32();
    let mut accumilator = 0.0;
    let mut new_frame = true;
    loop {
        let new_time = Instant::now();
        let frame_time = (new_time - current_time).as_secs_f32();
        current_time = new_time;

        accumilator += frame_time;

        while accumilator >= dt {
            movement_system(&mut world, dt);
            accumilator -= dt;
            new_frame = true;
        }
        if new_frame {
            render_system(&mut world);
            new_frame = false;
        }
    }
}

fn render_with_buffer(sender: Sender<Vec<u32>>) -> Box<dyn Fn(&mut World)> {
    Box::new(move |world| {
        let mut buffer = vec![0; WIDTH * HEIGHT];

        let positions = world.borrow_component_vec::<Position>().unwrap();
        for position in positions
            .iter()
            .filter_map(|position| Some(position.as_ref()?))
        {
            draw_as_rect(&mut buffer, position, 50, 50)
        }

        sender.send(buffer).unwrap();
    })
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
