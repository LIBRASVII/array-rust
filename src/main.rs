#![allow(unused)]
// use std::env;

// ON WORKING
// fn to change the array values
fn change_array() {
    let array: [f64; 2] = [0.2, 0.4];
    println!();
    println!("{:?}",array);
}

// fn with two ways to print the array
fn fn_array() {
    let array = ['l','i', 'u', 's'];

    println!("{:?}", array);
        
    for i in &array {
        print!("{}", i);
    }
}

fn main() {
    // array initialized with Data Type and 
    // Expression (the size of array)
    // and declared the data of array
    let array: [i32; 5] = [15, 20, 69, 87, 90];
    println!("Array - {:?}", array);
    println!("Array size - {}", array.len());

    // array without Data Type
    // only the data
    let array2 = ['A', 'B', 'C', 'D'];
    println!("Array2 - {:?}", array2);
    println!("Array2 size - {}", array2.len());

    // array initialized with all values default to zero 
    let array3: [i32; 5] = [0; 5];
    println!("Array3 - {:?}", array3);
    println!("Array3 size - {}", array3.len());

    // declare array4 with all default values to zero
    // after assign the values to array 
    let mut array4: [i32; 3] = [0; 3];
    array4[1] = 4;
    array4[2] = 6;

    // this is not working
    // assert_eq!([1, 2], &array4[1..]);
    // env::set_var("RUST_BRACKTRACE", "1");

    // loop how prints the values of array4
    for x in &array4 {
        println!("{x}\t");
    }

    // call the fn
    fn_array();

    change_array();
}
