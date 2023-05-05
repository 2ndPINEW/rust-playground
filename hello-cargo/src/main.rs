fn main() {
    println!("Hello, world!");
    let x = five();
    println!("Initialize x: {}", x);
    {
        let x = x + 5;
        println!("Scoped x: {}", x);
    }
    println!("Unscoped x: {}", x);
}

fn five() -> i32 {
    5
}
