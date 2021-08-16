//primitive str = immutable fixed length string somewhere in memory
//string = grownable, heap-allowed data structure - use when you need to modify or own string data

pub fn run()
{
    let mut hello = String::from("Hello ");


    println!("{}", hello);
    println!("Length: {}", hello.len());

    //add char
    hello.push('W');
    //add string
    hello.push_str("orld!");

    println!("{}", hello);
    println!("Length: {}", hello.len());

    //capacity in bytes / checks to is empty
    println!("Capacity: {}", hello.capacity());
    println!("IsEmpty: {}", hello.is_empty());

    //contains sub string / replace
    println!("Contain World: {}", hello.contains("World"));
    println!("Replace World: {}", hello.replace("World", "Corey"));

    //loop through string 
    for word in hello.split_whitespace()
    {
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');



    //assertion: testing - debugging
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}