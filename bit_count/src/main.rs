fn main(){
    println!("Hello, world!");
    let val: u32 = 10;
    let count = count_set_bit(val);
    println!("no of set bit is {}", count);
    

}

fn count_set_bit(val: u32) -> u16 {
    let mut _i = 0;
    let mut count_set:u16 = 0;
    println!("value received {}", val);
    for _i in 0..32 {
        if ((val>>_i) & 1) == 1 {
            count_set+=1;
        }
    }
    count_set
}