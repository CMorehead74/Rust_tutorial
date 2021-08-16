//Vectors are resizable arrays

use std::mem;

pub fn run()
{
    //fixed
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //add onto vector
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);
    numbers.pop();

    
    println!("{:?}", numbers);

    //get single value
    println!("Single value: {}", numbers[0]);

    //get vector leng
    println!("vector: {:?}", numbers.len());
    //vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice:
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //loop vector values
    for x in numbers.iter()
    {
        println!("Numbers: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("numbers: {:?}", numbers);
}