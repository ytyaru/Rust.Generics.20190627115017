/*
 * Rustのジェネリクス、トレイト、ライフタイム。
 * CreatedAt: 2019-06-27
 */
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    pub fn get_x(&self) -> &T { &self.x }
}
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let a = Point {x: 5, y: 6};
    println!("{}", a.get_x());
}

