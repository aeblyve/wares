use termion::terminal_size;

// nalgebra gives us this but eh 
struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {

    fn add(&mut self, other: Self) {
	self.x + other.x;
	self.y + other.y;
	self.z + other.z;
    }

    fn mul(&mut self, s: f64) {
	self.x *= s;
	self.y *= s;
	self.z *= s;
    }

    fn div(&mut self, s: f64) {
	self.mul(1.0/s);
    }

    fn neg(&mut self) {
	self.x = -self.x;
	self.y = -self.y;
	self.z = -self.z;
    }

    fn length(self) -> f64 {
	self.length_sqr().powf(0.5)
    }

    fn length_sqr(self) -> f64 {
	self.x * self.x + self.y * self.y + self.z * self.z
    }

}

fn main() {
    println!("Hello, world!");
    let size = terminal_size().unwrap();
    println!("Size: {} {}", size.0, size.1);

    loop {
	for row in 0..size.1 {
	    for col in 0..size.0 {
		// render at the pixeL!
	    }
	}
    }
}
