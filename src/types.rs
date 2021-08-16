//Primitive Types: 
//Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory) unsigned / int
//floats: f32, f64
//boolean: bool
//characters: char
//tuples
//arrays

//Rust is a statically typed language, whcich means that it must know the types of all variables at compule time, 
//however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run ()
{
    //default is "i32"
    let x = 1;
//default is "f64"
    let y = 2.5;

    //add explicit type
    let z: i64 = 4545454545;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //bool
    //let is_active = true; //<--> also good
    let is_active: bool = true;

    //get boolean from expression
    let is_greater = 10 < 5;

    //char
    let a1 = 'a'; //<--> any unicode char
    let face = '\u{1F600}'; //<--> smiley face

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}