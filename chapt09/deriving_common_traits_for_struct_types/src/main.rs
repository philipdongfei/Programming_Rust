#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}


fn main() {
    let p = Point { x: 1.0, y: 1.0 };
    let q = Point { x: 0.1, y: 1.0 };
    let pp = p;
    assert_eq!(p, pp);
    assert!(p != q);
    print!("{:?}\n",p);
    print!("{:?}\n",q);
}
