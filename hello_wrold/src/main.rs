use std::i8;

fn main() {
    println!("Hello, world!");

    println!("Will demo the datatypes");
    // variable
    let mut a:i16 = 10;
    println!("mutable a primary val: {a}");
    a = 20;
    println!("mutable a modified with val: {a}");

    // shadow, non mutable
    let b: i32 = 100;
    println!("non-mutable b primary val: {b}");
    let b: i32 = 200;
    println!("non-mutable b is shadow: and modified val: {b}");

    /* primitive data types */
    // bool
    let decision: bool = true;
    println!("decision bool : {decision}");
    // U integers
    let i1: u8 = 0xFF;
    let i2: u16 = 0xFFFF;
    let i3: u32 = 0xFFFFFFFF;
    let i4: u64 = 0xFFFFFFFFFFFFFFFF;
    let i5: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    // I integers
    let i6: i8 = 0x7F;
    let i7: i16 = 0x7FFF;
    let i8: i32 = 0x7FFFFFFF;
    let i9: i64 = 0x7FFFFFFFFFFFFFFF;
    let i10: i128 = 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    
    println!("i1: {i1}\ti2: {i2}\ti3: {i3}\ti4: {i4}\ti5: {i5}\t i6: {i6}\t i7: {i7}\ti8: {i8}\ti9: {i9}\ti10: {i10}\t");

    // Float , double
    let f:f32 = 60.25;
    let d:f64 =  70.3;
    println!("f: {f}\t d: {d}");

    // char and string
    let ch: char = 'a';
    println!("dec val: {ch}");
    let s1:&str = "type1 string";
    println!("s1: {s1}");
    let s2: String = String::from("type2 string");
    println!("s2: {s2}");

    /* Array */
    let arr:[u8; 5] = [1,2, 3, 4, 5];
    let arr_el: u8 = arr[4];
    println!("arr:[u8; 5] is arr_el: {arr_el}");

    /* tuple */
    println!("tuple can have any type of primitive datatype");
    let _tuple:(u8, u8) = (100, 200);
    //println!("same datatype tuple: {tuple.0}, {tuple.1}");
    let tuple:(u8, f64, &str) = (10, 10.23, "Mix Tuple");

    let str: &str = tuple.2;
    println!("str from tuple: {str}");
    /* extract complete tuple */
    let (u8, f1, s1) = tuple;
    println!("From tuple: {u8}, {f1}, {s1}");

    /* Empty tuple, Usage, return by void func(...) */
    let _unit = ();

    /* type aliasing more like */
    println!("type aliasing more like typedef");
    type Age = u8;
    let a1:Age = 60;
    println!("a1 : {a1}");

}
