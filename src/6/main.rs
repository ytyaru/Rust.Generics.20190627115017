/*
 * Rustのジェネリクス、トレイト、ライフタイム。
 * CreatedAt: 2019-06-27
 */
#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}
impl<T,U> Point<T,U> {
    pub fn mixup<V,W>(self, other: Point<V, W>) -> Point<T,W> { Point { x: self.x, y: other.y } }
}
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: "A", y: "B" };
    let p3 = p1.mixup(p2);
//    println!("{:?}", p1); // error[E0382]: use of moved value: `p1`
//    println!("{:?}", p2); // error[E0382]: use of moved value: `p2`
    println!("{:?}", p3);
}

