use std::io::{stdin, stdout, Read, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::process;
use std::{thread, time};
use termion::async_stdin;
use termion::event::Key;
use termion::input::Keys;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen, ToMainScreen};
use termion::terminal_size;

/// A nice 64 colors == 6 bits
const BRIGHTNESS_MAP: &str = "WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

//https://docs.rs/termion/latest/termion/raw/struct.RawTerminal.html

struct Ray3 {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray3 {
    fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

// nalgebra gives us this but eh
#[derive(Clone, Copy)]
struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self.mul(1.0 / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs);
    }
}

impl Vec3 {
    fn unit(self) -> Self {
        let len = self.length();
        self / len
    }

    fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length(&self) -> f64 {
        self.length_sqr().powf(0.5)
    }

    fn length_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

fn hit_sphere(center: Vec3, radius: f64, r: Ray3) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant > 0.
}

/// Where 0.0 \leq brightness \leq 1.0
fn brightness_to_char(brightness: f64) -> char {
    BRIGHTNESS_MAP
        .chars()
        .nth(((brightness * BRIGHTNESS_MAP.len() as f64) as usize - 1))
        .unwrap()
}

fn render_ray(r: Ray3) -> char {
    let unit_dir = r.direction.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    brightness_to_char((1.0 - t) * 1.0 + t * 0.5)
}

fn main() {
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let mut stdin = async_stdin().bytes();
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    screen.flush().unwrap();

    let one_sec = time::Duration::from_millis(1000);
    let frame_timing = time::Duration::from_millis(16); // roughly 1/60th of second

    let size = terminal_size().unwrap();

    let aspect_ratio = size.0 as f64 / size.1 as f64;
    // width and height are terminal

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.,
        z: 0.,
    };
    let vertical = Vec3 {
        x: 0.,
        y: viewport_height,
        z: 0.,
    };

    let lower_left_corner = origin
        - horizontal / 2.
        - vertical / 2.
        - Vec3 {
            x: 0.,
            y: 0.,
            z: focal_length,
        };

    loop {
        let c = stdin.next();
        if let Some(Ok(b'q')) = c {
            break;
        }
        write!(screen, "{}", termion::clear::All);
        screen.flush().unwrap();
        // for some reason ANSI escapes are one-based.
        for row in 1..=size.1 {
            for col in 1..=size.0 {
                let u = col as f64 / size.0 as f64;
                let v = row as f64 / size.1 as f64;
                let r = Ray3 {
                    origin: origin,
                    direction: lower_left_corner + horizontal * u + vertical * v - origin,
                };
                write!(
                    screen,
                    "{}{}",
                    termion::cursor::Goto(col, row),
                    if hit_sphere(
                        Vec3 {
                            x: 0.,
                            y: 0.,
                            z: -1.
                        },
                        0.5,
                        r
                    ) {
                        "'"
                    } else {
                        "W"
                    } //render_ray(r)
                );
                screen.flush().unwrap();
            }
        }
        thread::sleep(frame_timing);
    }
    write!(screen, "{}", termion::cursor::Show).unwrap();
}
