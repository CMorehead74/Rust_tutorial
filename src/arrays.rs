//Arrays = fixed list where elements are the same data types

use std::mem;

pub fn run()
{
    //fixed
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("{:?}", numbers);

    //get single value
    println!("Single value: {}", numbers[0]);

    let mut mut_numbers: [i32; 5] = [1, 2, 3, 4, 5];

    mut_numbers[2] = 20;

    println!("Muttabe array: {:?}", mut_numbers);

    //get array leng
    println!("Muttabe array: {:?}", mut_numbers.len());
    //arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice:
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}