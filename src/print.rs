pub fn run()
{
    //Print to console
    println!("!Hello from the print.rs file");

    //printing numbers
    println!("Number: {}", 1);

    //print multi
    println!("{} is from {}", "Corey", "Winter Garden");

    //print positional arguments
    println!("{0} is from {1} and {0} likes to play {2}", "Corey", "Winter Garden", "D&D");

    //print Named Arguments
    println!("{name} likes to play {game}", name = "Corey", game = "D&D");

    //print Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //print placeholder for debug trait (tuple)
    println!("{:?}", (12, true, "hello"));

    //print basic math
    println!("10 + 10 = {}", 10 + 10);
}