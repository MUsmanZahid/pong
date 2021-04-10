//! We only support 64-bit architectures
#![cfg(target_pointer_width = "64")]

mod input {
    #![allow(dead_code)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum Key {
        ArrowLeft,
        ArrowRight,
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
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    #[derive(Clone, Copy, Debug)]
    pub struct Vector2 {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    pub struct Vector4 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }

    impl Add for Vector4 {
        type Output = Self;

        fn add(mut self, other: Self) -> Self::Output {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
            self.w += other.w;

            return self;
        }
    }

    impl AddAssign for Vector4 {
        fn add_assign(&mut self, other: Self) {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
            self.w += other.w;
        }
    }

    impl Add<f32> for Vector4 {
        type Output = Self;

        fn add(mut self, other: f32) -> Self::Output {
            self.x += other;
            self.y += other;
            self.z += other;
            self.w += other;

            return self;
        }
    }

    impl AddAssign<f32> for Vector4 {
        fn add_assign(&mut self, other: f32) {
            self.x += other;
            self.y += other;
            self.z += other;
            self.w += other;
        }
    }

    impl Div for Vector4 {
        type Output = Self;

        fn div(mut self, other: Self) -> Self::Output {
            self.x /= other.x;
            self.y /= other.y;
            self.z /= other.z;
            self.w /= other.w;

            return self;
        }
    }

    impl DivAssign for Vector4 {
        fn div_assign(&mut self, other: Self) {
            self.x /= other.x;
            self.y /= other.y;
            self.z /= other.z;
            self.w /= other.w;
        }
    }

    impl Div<f32> for Vector4 {
        type Output = Self;

        fn div(mut self, other: f32) -> Self::Output {
            self.x /= other;
            self.y /= other;
            self.z /= other;
            self.w /= other;

            return self;
        }
    }

    impl DivAssign<f32> for Vector4 {
        fn div_assign(&mut self, other: f32) {
            self.x /= other;
            self.y /= other;
            self.z /= other;
            self.w /= other;
        }
    }

    impl Mul for Vector4 {
        type Output = Self;

        fn mul(mut self, other: Self) -> Self::Output {
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
            self.w *= other.w;

            return self;
        }
    }

    impl MulAssign for Vector4 {
        fn mul_assign(&mut self, other: Self) {
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
            self.w *= other.w;
        }
    }

    impl Mul<f32> for Vector4 {
        type Output = Self;

        fn mul(mut self, other: f32) -> Self::Output {
            self.x *= other;
            self.y *= other;
            self.z *= other;
            self.w *= other;

            return self;
        }
    }

    impl MulAssign<f32> for Vector4 {
        fn mul_assign(&mut self, other: f32) {
            self.x *= other;
            self.y *= other;
            self.z *= other;
            self.w *= other;
        }
    }

    impl Sub for Vector4 {
        type Output = Self;

        fn sub(mut self, other: Self) -> Self::Output {
            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
            self.w -= other.w;

            return self;
        }
    }

    impl SubAssign for Vector4 {
        fn sub_assign(&mut self, other: Self) {
            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
            self.w -= other.w;
        }
    }

    impl Sub<f32> for Vector4 {
        type Output = Self;

        fn sub(mut self, other: f32) -> Self::Output {
            self.x -= other;
            self.y -= other;
            self.z -= other;
            self.w -= other;

            return self;
        }
    }

    impl SubAssign<f32> for Vector4 {
        fn sub_assign(&mut self, other: f32) {
            self.x -= other;
            self.y -= other;
            self.z -= other;
            self.w -= other;
        }
    }
}

mod ffi;
mod gfx;
mod window;

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

fn main() {
    let mut window = Window::new("Pong!", "rose", 800, 600);
    let mut renderer = Renderer::init(&window);
    let delta = 1.5E-2;

    let mut position = Vector2 { x: 0.0, y: 0.5 };
    let paddle = renderer.create_sprite("textures/paddle.png", position);
    let ball = renderer.create_sprite("textures/ball.png", position);

    while !window.exiting {
        while let Some(event) = window.poll_event() {
            match event {
                Event::KeyPress(Key::ArrowLeft) => {
                    let (hw, _) = renderer.sprite_half_dimensions(paddle);
                    position.x = f32::clamp(position.x - delta, -1.0 + hw, 1.0 - hw);
                }
                Event::KeyPress(Key::ArrowRight) => {
                    let (hw, _) = renderer.sprite_half_dimensions(paddle);
                    position.x = f32::clamp(position.x + delta, -1.0 + hw, 1.0 - hw);
                }
                Event::WindowFocused => {
                    std::process::Command::new("xset")
                        .args(&["r", "rate", "10", "25"])
                        .status()
                        .unwrap();
                }
                Event::WindowUnfocused => {
                    std::process::Command::new("xset")
                        .args(&["r", "rate"])
                        .status()
                        .unwrap();
                }
                _ => {}
            }
        }

        if let Some(index) = renderer.begin_scene(0.7, 0.4, 0.8) {
            renderer.draw(paddle, Vector2 { x: 0.0, y: -0.5 });
            renderer.draw(ball, Vector2 { x: 0.0, y: 0.0 });
            renderer.draw(paddle, position);
            renderer.end_scene();
            renderer.present(index);
        } else {
            renderer.resize(&window);
        }
    }

    renderer.deinit();
}
