fn main() {
    let a: i32 = 1;
    let b: i32 = 2;
    println!("1 + 2 = {} even that way", add(&a, &b));
}

fn add(a: &i32, b: &i32) -> i32 {
    *a + *b
}