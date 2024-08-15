fn main() {
    //Primatives data types

    //Integer
    //Rust has signed (+/-) and unsigned (+/-) integers of various sizes.
    //Signed integers are represented by the type iN, where N is the number of bits in the integer.
    //Unsigned integers are represented by the type uN, where N is the number of bits in the integer.
    //i8,i16,i32,i64,i128 : signed integers
    //u8,u16,u32,u64,u128 : unsigned integers
    let x: i32 = -42; //signed integer
    let y: u64 = 42; //unsigned integer
    let e: i32 = 0x7FFFFFFF; //signed integer
    let f: u32 = 0xFFFFFFFF; //unsigned integer
    let g: i64 = 0x7FFFFFFFFFFFFFFF; //signed integer
    println!(
        "i32 Signed Int = {} maxvalue = {} u32 Unsigned Int = {} maxvalue={} i64 {}",
        x, e, y, f, g
    );

    //Float f32 and f64
    let a: f32 = 3.14; //32 bit floating point

    println!("Value of pi is {}", a);


    //Boolean
    let b: bool = true;
    println!("Is it True?:  b is {}", b);

    //Character
    let c: char = 'a';
    println!("Character is {}", c);

    //String
    let s: String = String::from("Hello World");
    let h: String = String::from("Hello World String with emojii 🦀");
    println!("String is {}", s);
    println!("String w/Emoji is {}", h);
}