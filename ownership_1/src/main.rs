fn main() {
    println!("Hello, world!");
    let x = 10;
    println!("value of x: {x}");
    let mut y = x;
    y += 20;
    println!("value of x after move: {x}");
    println!("value of y: {y}");

    let str_1 = String::from("Rust");
    let str_2 = String::from("Clone Rust");
    println!("str_1: {str_1} \tstr_2: {str_2}");
    let str_3 = str_1;
    //println!("value of str_1: {str_1}");    // error value borrowed here after move
    println!("str_3: {str_3}");
    let str_4 = str_2.clone();
    println!("clone str_2 to str_4");    // error value borrowed here after move
    println!("str_2 after cloning: {str_2}\t str_4: {str_4}");
}
