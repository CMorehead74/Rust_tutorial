//Variables hold primitive data or reference to data
//Variables are immutable by default (similar to const)
//Rust is a block-scoped language

pub fn run() 
{
    let name = "Corey";
    let mut age = 46;
    println!("My name is {} and I was {} yesterday", name, age);
    age = 47;
    println!("My name is {} and I am {} today", name, age);

    //const 
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //mult variables: 
    let (my_name, my_age) = ("Corey", 47);
    println!("My name is {} and my age {}", my_name, my_age);
}