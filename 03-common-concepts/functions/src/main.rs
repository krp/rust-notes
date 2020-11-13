fn main() {
    println!("Hello, world!");
    my_func();
    let x = five();
    println!("done, {}", x);
}

fn my_func() {
    println!("hello from inside my_func");
    42;
}

fn five() -> i32 { 5 }
