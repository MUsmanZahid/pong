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
    use std::ops::{AddAssign, Neg};

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

    impl Neg for Vector2 {
        type Output = Self;

        fn neg(mut self) -> Self::Output {
            self.x = -self.x;
            self.y = -self.y;
            return self;
        }
    }
}

mod ffi;
mod font;
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
    half_width: f32,
    half_height: f32,
    handle: usize,
}

impl Ball {
    fn centered(renderer: &mut Renderer, velocity: Vector2) -> Self {
        let position = Vector2 { x: 0.0, y: 0.0 };
        let handle = renderer.create_sprite_from_path("textures/ball.png");
        let (half_width, half_height) = renderer.sprite_half_dimensions(handle);
        let ball = Self {
            position,
            velocity,
            half_width,
            half_height,
            handle,
        };
        return ball;
    }

    fn simulate(&mut self, top_paddle: &Paddle, bottom_paddle: &Paddle) -> bool {
        self.position += self.velocity;

        // Modelling top-paddle interaction
        let ball_bottom_edge = self.position.y - self.half_height;
        let ball_top_edge = self.position.y + self.half_height;
        let ball_right_edge = self.position.x + self.half_width;
        let ball_left_edge = self.position.x - self.half_width;

        let top_paddle_bottom_edge = top_paddle.position.y - top_paddle.half_height;
        let top_paddle_top_edge = top_paddle.position.y + top_paddle.half_height;
        let top_paddle_right_edge = top_paddle.position.x + top_paddle.half_width;
        let top_paddle_left_edge = top_paddle.position.x - top_paddle.half_width;

        let bottom_paddle_bottom_edge = bottom_paddle.position.y - bottom_paddle.half_height;
        let bottom_paddle_top_edge = bottom_paddle.position.y + bottom_paddle.half_height;
        let bottom_paddle_right_edge = bottom_paddle.position.x + bottom_paddle.half_width;
        let bottom_paddle_left_edge = bottom_paddle.position.x - bottom_paddle.half_width;

        if self.velocity.y > 0.0 {
            if top_paddle_bottom_edge <= ball_top_edge && ball_top_edge <= top_paddle_top_edge {
                if ball_right_edge >= top_paddle_left_edge
                    && ball_left_edge <= top_paddle_right_edge
                {
                    self.velocity.y = -self.velocity.y;
                    return false;
                }
            }
        } else {
            if bottom_paddle_top_edge >= ball_bottom_edge
                && ball_bottom_edge >= bottom_paddle_bottom_edge
            {
                if ball_right_edge >= bottom_paddle_left_edge
                    && ball_left_edge <= bottom_paddle_right_edge
                {
                    self.velocity.y = -self.velocity.y;
                    return false;
                }
            }
        }

        // Play area's vertical bounds were exceeded
        if self.position.y <= -1.0 + self.half_height {
            self.velocity.y = -self.velocity.y;
            return true;
        } else if self.position.y >= 1.0 - self.half_height {
            self.velocity.y = -self.velocity.y;
            return true;
        }

        // Play area's horizontal bounds were exceeded
        if self.position.x <= -1.0 + self.half_width {
            self.velocity.x = -self.velocity.x;
        } else if self.position.x >= 1.0 - self.half_width {
            self.velocity.x = -self.velocity.x;
        }

        return false;
    }
}

struct Paddle {
    position: Vector2,
    half_width: f32,
    half_height: f32,
    handle: usize,
}

impl Paddle {
    fn bottom(handle: usize, (half_width, half_height): (f32, f32)) -> Self {
        let position = Vector2 {
            x: 0.0,
            y: -1.0 + half_height,
        };
        let paddle = Self {
            position,
            half_width,
            half_height,
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
        let position = Vector2 {
            x: 0.0,
            y: 1.0 - half_height,
        };
        let paddle = Self {
            position,
            half_width,
            half_height,
            handle,
        };
        return paddle;
    }
}

#[derive(Debug)]
enum GameState {
    Neutral,
    SetActive,
}

macro_rules! cstr {
    ( $s:literal ) => {{
        let s = concat!($s, "\0");
        std::ffi::CStr::from_bytes_with_nul(s.as_bytes()).unwrap()
    }};
}

fn main() {
    let font_path = cstr!("/usr/share/fonts/TTF/Comfortaa-Light.ttf");
    let (_glyphs, _coverage) = font::generate_bitmap(font_path, 64);

    let mut window = Window::new("Pong!", "rose", 800, 600);
    let mut renderer = Renderer::init(&window);

    // renderer.draw_text_centered("Usman", Vector2 { x: -1.0, y: 0.0 });

    let delta = 3.0E-2;
    let (mut bottom_paddle, mut top_paddle) = {
        let handle = renderer.create_sprite_from_path("textures/paddle.png");
        let half_dimensions = renderer.sprite_half_dimensions(handle);
        let bottom = Paddle::bottom(handle, half_dimensions);
        let top = Paddle::top(handle, half_dimensions);
        (bottom, top)
    };
    let mut ball = Ball::centered(
        &mut renderer,
        Vector2 {
            x: 1.0E-2,
            y: 9.0E-3,
        },
    );

    let mut state = GameState::Neutral;
    while !window.exiting {
        while let Some(event) = window.poll_event() {
            match state {
                GameState::Neutral => {
                    if let Event::KeyPress(Key::Enter) = event {
                        state = GameState::SetActive;
                        set_keyboard_delay_and_repeat(Some(Duration::from_millis(10)), Some(25));
                    } else if let Event::WindowUnfocused = event {
                        set_keyboard_delay_and_repeat(None, None);
                    }
                }
                GameState::SetActive => match event {
                    Event::KeyPress(Key::ArrowLeft) => bottom_paddle.move_left(delta),
                    Event::KeyPress(Key::ArrowRight) => bottom_paddle.move_right(delta),
                    Event::KeyPress(Key::A) => top_paddle.move_left(delta),
                    Event::KeyPress(Key::D) => top_paddle.move_right(delta),
                    Event::WindowUnfocused => set_keyboard_delay_and_repeat(None, None),
                    _ => {}
                },
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

        if let GameState::SetActive = state {
            if ball.simulate(&top_paddle, &bottom_paddle) {
                state = GameState::Neutral;

                ball.velocity = -ball.velocity;
                ball.position = Vector2 { x: 0.0, y: 0.0 };
                top_paddle.position.x = 0.0;
                bottom_paddle.position.x = 0.0;

                set_keyboard_delay_and_repeat(None, None);
            }
        }
    }

    renderer.deinit();
}
