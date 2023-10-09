
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // 因为在模式匹配代码块中，我们没有机会声明变量类型，只能用 ref 表示变量 t 是个指针或者直接传入一个指针
    match &y {
        Some( p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
