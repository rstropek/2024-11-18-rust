struct Vector2d {
    x: f64,
    y: f64,
}

impl Vector2d {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn get_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let vectors = vec![Vector2d::new(10.0, 20.0), Vector2d::new(3.0, 4.0)];

    let vectors_iter = vectors.iter();
    let total_length = vectors_iter.map(|v| v.get_length()).sum::<f64>();

    println!("Total length: {}", total_length as i32);
}
