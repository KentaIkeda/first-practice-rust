fn main() {
    let result: i32 = add(1, 2);
    println!("result: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}