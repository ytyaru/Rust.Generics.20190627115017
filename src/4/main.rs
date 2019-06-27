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
fn main() {
    let a = Point {x: 5, y: 6};
    println!("{}", a.get_x());
}

