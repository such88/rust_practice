fn main() {
    println!("Hello, world!");

    let start = 2;
    let end = 100;
    //let no_of_count = 
    println!("no of prime is {}", print_count_prime(start, end));
}

fn print_count_prime(mut start:u32, end: u32) ->u32 {
    let mut _i = 1;
    let mut div_times = 0;
    let mut count_prime = 0;

    while start <=end {
        for _i in 1..(start+1) {
            if (start%_i) ==0 {
                div_times +=1;
            }
        }
        if div_times == 2 {
            count_prime += 1;
            println!("{} is prime", start);
        }
        start+= 1;
        div_times = 0;
    }

    count_prime
}
