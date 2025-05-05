//use num::integer::sqrt;

fn main() {
    println!("Hello, world!");

    let ip_1:u32 = 999999999;
    let ip_2:u32 = 100000000;
    let x:bool = is_ip_1_is_bigger(ip_1, ip_2);

    println!{"x is : {}", x};

    let numset = [36, 25, 49, 3, 64, 16, 9];
    let prime  = get_prime_number(numset);

    println!{"first prime num: {}", prime};

}

fn is_ip_1_is_bigger(a: u32, b: u32) -> bool {
    if a>b {true} else {false}
}

fn get_prime_number(arr: [i32; 7]) -> i32 {
    let mut i =0;
    'outer:loop {
        let mut n = 2;
        loop{
            if arr[i]%n == 0 {
                if arr[i] == 2 {
                    break 'outer;
                }
                i+=1;
                break;
            }

            if n >= (arr[i]/2) {
                break 'outer;
            }

            n += 1;
        }
    }
    println!("The first prime number in the array is {}.", arr[i]);
    arr[i]
}