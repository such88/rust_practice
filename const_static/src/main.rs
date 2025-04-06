static BEACH_NAME: &str = "GOA";
fn main() {
    println!("Hello, world!");

    const MAX_LEN:i32 = 10;
    println!("you can't jump more than {} feet", MAX_LEN);

    let mut best_beach: &str = BEACH_NAME;
    println!("Best beach in india is {}", best_beach);
    best_beach = "West indies";
    println!("Best beach in world is {}", best_beach);
}

