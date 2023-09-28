#[derive(Default, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new(points: Vec<Point>) -> Result<Self, String> {
        if points.is_empty() {
            return Err("Polyline must contain at least 1 point".to_string());
        }

        Ok(Polyline { points })
    }
}

fn main() {}
