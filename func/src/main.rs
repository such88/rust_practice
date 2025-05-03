fn main() {
    println!("Hello, world!");
    let ret:i8 = my_function(11);

    println!("ret value of my_function: {}", ret);
}

fn my_function(x: i32) -> i8{
    println!{"my_function called with: {}", x}
    let y: i8 = 10;
    y
}
