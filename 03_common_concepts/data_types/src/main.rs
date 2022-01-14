fn main() {
    //--scalar types
    //interger (u8/16/32/62/128 , i8/16/32/64/128 , usize, isize)
    let int: u8 = 2; 

    //floating-point
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //Numeric opertations
    let z = 5 + 30;
    println!("{}", z);

    //boolean
    let t = true;
    let f: bool = false;

    //char
    let cat = '😻';

    //--compound type
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    let five_hundred = tup.0;

    //array
    let arr: [u8; 5] = [1,2,3,4,5];
    let first_element = arr[0];
}


// This code checks what's going on if there is an invalid access of array
// use std::io;

// fn main() {
//     let a = [1,2,3,4,5];

//     println!("Please give item to array!");
//     let mut index = String::new();
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index.trim().parse().expect("please give me number");

//     let element = a[index];

// }