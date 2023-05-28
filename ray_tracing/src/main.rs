use std::io::{stdin, stdout, Read, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};
use std::process;
use termion::async_stdin;
use termion::event::Key;
use termion::input::Keys;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
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
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    write!(stdout, "{}", termion::clear::All);
    stdout.flush().unwrap();
    let size = terminal_size().unwrap();
    // println!("Size: {} {}", size.0, size.1);

    loop {
        let c = stdin.next();
        if let Some(Ok(b'q')) = c {
            break;
        }
        // for c in stdin.keys() {
        //     match c.unwrap() {
        //         Key::Char('q') => process::exit(0), // reset terminal or whatever
        //         _ => {println!("")}
        //     }
        // }
        for row in 0..size.1 {
            // https://docs.rs/termion/latest/termion/fn.async_stdin.html
            for col in 0..size.0 {
                termion::cursor::Goto(row, col);
                write!(stdout, "#");
                stdout.flush().unwrap();
                // render at the pixeL!
            }
        }
    }
}
