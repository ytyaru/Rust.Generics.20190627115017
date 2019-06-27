/*
 * Rustのジェネリクス、トレイト、ライフタイム。
 * CreatedAt: 2019-06-27
 */
fn main() {
    let v = vec![3, 10, 2, 1, 9];
    /*
    let mut max = v[0];
    for i in &v {
        if max < *i { max = *i; }
    }
    println!("{:?}: {}", v, max);
    */
    println!("{:?}: {}", v, get_max(&v));
}
fn get_max(list: &[T]) -> T { // error[E0412]: cannot find type `T` in this scope
    let mut max = list[0];
    for i in list {
        if max < *i { max = *i; }
    }
    max
}
