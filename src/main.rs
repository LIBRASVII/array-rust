#![allow(unused)]

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


    let mut array4: [i32; 2] = [0, 2];
    array4[0] = 4;
    array4[1] = 6;
    // assert_eq!([0, 1], &array4[0..]);

    for x in &array4 {
        print!("{x}\t");
    }


}
