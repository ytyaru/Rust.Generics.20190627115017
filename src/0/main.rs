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
fn get_max(v: &Vec<i32>) -> i32 {
    let mut max = v[0];
    for i in v {
        if max < *i { max = *i; }
    }
    max
}
