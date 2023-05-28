use std::io::{stdin, stdout, Read, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};
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

// nalgebra gives us this but eh
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

fn main() {
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    let mut stdin = async_stdin().bytes();
    //write!(screen, "{}", termion::cursor::Hide).unwrap();
    screen.flush().unwrap();

    let one_sec = time::Duration::from_millis(1000);

    let size = terminal_size().unwrap();

    loop {
        let c = stdin.next();
        if let Some(Ok(b'q')) = c {
            break;
        }
        write!(screen, "{}", termion::clear::All);
        screen.flush().unwrap();
        // for some reason ANSI escapes are one-based, and column-major-ordered..
        for row in 1..=size.1 {
            for col in 1..=size.0 {
                write!(screen, "{}", termion::cursor::Goto(col, row));
                screen.flush().unwrap();
                //termion::cursor::Goto(row, col);
                //write!(stdout, "#");
                //stdout.flush().unwrap();
            }
        }
    }
    //write!(screen, "{}", termion::cursor::Show).unwrap();
}
