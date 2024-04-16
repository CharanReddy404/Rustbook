fn main() {
    // // mutiable
    // let mut x = 5;
    // println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}");
    // x = 7;
    // println!("The value of x is {x}");

    // // Shadowing variables

    // let x = 5;
    // println!("The value of x is: {x}");

    // let x = 2 + 1;
    // println!("The value of x is: {x}");

    // let x = 2 + 45;
    // println!("The value of x is: {x}");

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // // mut vs shadow
    // //shadow
    // let x = "abcd";
    // println!("The value of x is: {x}");

    // let x = x.len();
    // println!("The value of x is: {x}");

    // println!("---------------------------");
    // //mut - datatype cant be changes
    // let mut x = "abcd";
    // println!("The value of x is: {x}");

    // x = "xyz";
    // println!("The value of x is: {x}");

    // //error because data type cannot be changed
    // // x = x.len();
    // // println!("The value of x is: {x}");

    // Type annotations
    // this will give an error because we should tell the compailer what datatype are parsing(Type annotations)
    // let guess = "23".parse().expect("Not a number!");

    // let guess: u32 = "23".parse().expect("Not a number!");
    // println!("{guess}");

    // two data type subsets: scalar and compound.

    // Scalar has 4 type
    // 1. integers
    // 2. floating-point numbers
    // 3. Booleans
    // 4. characters

    // Length	  Signed	Unsigned
    // 8-bit	  i8	    u8
    // 16-bit	  i16	    u16
    // 32-bit	  i32	    u32
    // 64-bit	  i64	    u64
    // 128-bit	  i128	    u128
    // arch	      isize	    usize

    // println!("{}", u32::MAX); // 4294967295
    // println!("{}", u32::MIN); // 0
    // println!("{}", i32::MAX); // 2147483647
    // println!("{}", i32::MIN); // -2147483648

    // let num: u8 = 256;
    // println!("{num}")

    // floting point
    // let x = -2.0; // f64

    // let y: f32 = 3.0; // f32
    // println!("{x} <=> {y}")

    // Compound Types
    // Rust has two primitive compound types: tuples and arrays.

    // tuples
    // let tup = (500, 6.4, 1);

    // let (_x, y, _z) = tup;

    // println!("The value of y is: {y}");

    // let x: (i32, f64, u8, &str) = (500, 6.4, 1, "hello");

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // let str: &str = x.3;

    // println!("five_hundred: {five_hundred}");
    // println!("six_point_four: {six_point_four}");
    // println!("one: {one}");
    // println!("str: {str}");

    // Array Type
    // let a: [u32; 6] = [1, 2, 3, 4, 5, 6];

    // let a = [3; 5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
