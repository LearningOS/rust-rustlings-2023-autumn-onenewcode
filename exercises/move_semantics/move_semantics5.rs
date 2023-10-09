

fn main() {
    let mut x = 100;
    // let y = &mut x;
    let z = &mut x;
    // *y += 100;
    *z += 1100;
    assert_eq!(x, 1200);
}
