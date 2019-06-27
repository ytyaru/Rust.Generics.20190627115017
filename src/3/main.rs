/*
 * Rustのジェネリクス、トレイト、ライフタイム。
 * CreatedAt: 2019-06-27
 */
struct Point<T> {
    x: T,
    y: T,
}
struct Point2<T,U> {
    x: T,
    y: U,
}
fn main() {
    let a = Point {x: 0, y: 0};
    let b = Point {x: 0.0, y: 0.0};
//    let c = Point {x: 0, y: 0.0}; // error[E0308]: mismatched types
    let d = Point2 {x: 0, y: 0.0};
}

