fn main() {
    println!("Hello, world!");

    let s = outer();
}

fn outer() -> fn() {
    let s = "ssss".to_string();

    fn inner() {
        println!("{s}");
    }

    return inner;
}
