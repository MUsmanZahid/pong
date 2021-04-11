//! We only support 64-bit architectures
#![cfg(target_pointer_width = "64")]

mod input {
    #![allow(dead_code)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum Key {
        ArrowLeft,
        ArrowRight,
        A,
        D,
        Enter,
        Unknown,
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum Event {
        KeyPress(Key),
        KeyRelease(Key),
        WindowFocused,
        WindowUnfocused,
        WindowResize(u16, u16),
    }
}

mod math {
    use std::ops::AddAssign;

    #[derive(Clone, Copy, Debug)]
    pub struct Vector2 {
        pub x: f32,
        pub y: f32,
    }

    impl AddAssign for Vector2 {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }
}

mod ffi;
mod gfx;
mod window;

use std::time::Duration;

use gfx::Renderer;
use input::{Event, Key};
use math::Vector2;
use window::Window;

fn read_png<P: AsRef<std::path::Path>>(path: P) -> (u32, u32, Box<[u8]>) {
    let decoder = png::Decoder::new(std::fs::File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buffer = vec![0; info.buffer_size()].into_boxed_slice();
    reader.next_frame(&mut buffer).unwrap();

    return (info.width, info.height, buffer);
}

#[cfg(target_os = "linux")]
fn set_keyboard_delay_and_repeat(delay: Option<Duration>, repeat: Option<u32>) {
    match (delay, repeat) {
        (Some(d), Some(r)) => {
            let string_delay = format!("{}", d.as_millis());
            let string_repeat = format!("{}", r);

            std::process::Command::new("xset")
                .args(&["r", "rate", &string_delay, &string_repeat])
                .status()
                .unwrap();
        }
        (Some(d), None) => {
            let string_delay = format!("{}", d.as_millis());
            std::process::Command::new("xset")
                .args(&["r", "rate", &string_delay])
                .status()
                .unwrap();
        }
        (None, Some(_)) => {}
        (None, None) => {
            std::process::Command::new("xset")
                .args(&["r", "rate"])
                .status()
                .unwrap();
        }
    }
}

struct Ball {
    position: Vector2,
    velocity: Vector2,
    handle: usize,
}

impl Ball {
    fn centered(renderer: &mut Renderer, velocity: Vector2) -> Self {
        let position = Vector2 { x: 0.0, y: 0.0 };
        let handle = renderer.create_sprite("textures/ball.png", position);
        let ball = Self {
            position,
            velocity,
            handle,
        };
        return ball;
    }

    fn simulate(&mut self) {
        self.position += self.velocity;
    }
}

struct Paddle {
    position: Vector2,
    half_width: f32,
    _half_height: f32,
    handle: usize,
}

impl Paddle {
    fn bottom(handle: usize, (half_width, half_height): (f32, f32)) -> Self {
        let position = Vector2 { x: 0.0, y: -0.5 };
        let paddle = Self {
            position,
            half_width,
            _half_height: half_height,
            handle,
        };
        return paddle;
    }

    fn move_left(&mut self, delta: f32) {
        self.position.x = f32::clamp(
            self.position.x - delta,
            -1.0 + self.half_width,
            1.0 - self.half_width,
        );
    }

    fn move_right(&mut self, delta: f32) {
        self.position.x = f32::clamp(
            self.position.x + delta,
            -1.0 + self.half_width,
            1.0 - self.half_width,
        );
    }

    fn top(handle: usize, (half_width, half_height): (f32, f32)) -> Self {
        let position = Vector2 { x: 0.0, y: 0.5 };
        let paddle = Self {
            position,
            half_width,
            _half_height: half_height,
            handle,
        };
        return paddle;
    }
}

fn main() {
    let mut window = Window::new("Pong!", "rose", 800, 600);
    let mut renderer = Renderer::init(&window);
    let delta = 1.5E-2;

    let (mut bottom_paddle, mut top_paddle) = {
        let handle = renderer.create_sprite("textures/paddle.png", Vector2 { x: 0.0, y: 0.0 });
        let half_dimensions = renderer.sprite_half_dimensions(handle);
        let bottom = Paddle::bottom(handle, half_dimensions);
        let top = Paddle::top(handle, half_dimensions);
        (bottom, top)
    };
    let mut ball = Ball::centered(&mut renderer, Vector2 { x: 0.0, y: 5.0E-3 });

    while !window.exiting {
        while let Some(event) = window.poll_event() {
            match event {
                Event::KeyPress(Key::ArrowLeft) => bottom_paddle.move_left(delta),
                Event::KeyPress(Key::ArrowRight) => bottom_paddle.move_right(delta),
                Event::KeyPress(Key::A) => top_paddle.move_left(delta),
                Event::KeyPress(Key::D) => top_paddle.move_right(delta),
                Event::WindowFocused => {
                    set_keyboard_delay_and_repeat(Some(Duration::from_millis(10)), Some(25));
                }
                Event::WindowUnfocused => {
                    set_keyboard_delay_and_repeat(None, None);
                }
                _ => {}
            }
        }

        if let Some(index) = renderer.begin_scene(0.7, 0.4, 0.8) {
            renderer.draw(bottom_paddle.handle, bottom_paddle.position);
            renderer.draw(ball.handle, ball.position);
            renderer.draw(top_paddle.handle, top_paddle.position);
            renderer.end_scene();
            renderer.present(index);
        } else {
            renderer.resize(&window);
        }

        ball.simulate();
    }

    renderer.deinit();
}
