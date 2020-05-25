pub struct Point2d {
    x: f32,
    y: f32,
}

impl Point2d {
    pub fn zero() -> Point2d {
        Point2d {
            x: 0.0_f32,
            y: 0.0_f32,
        }
    }

    pub fn from(x: f32, y: f32) -> Point2d {
        Point2d { x, y }
    }

    pub fn dist(&self, other: &Point2d) -> f32 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

fn main() {
    let pt1 = Point2d::zero();
    let pt2 = Point2d::from(10.0, 20.0);
    println!("{:?} {:?}", pt1, pt2);
    println!("{}", pt1.dist(&pt2));
}
