use std::env;

pub fn run()
{
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Corey";
    let status = "awesome!";

    println!("Args: {:?}", args); //pass in hello
    println!("Command: {}", command); //pass in hello (creates a command)

    if command == "hello" 
    {
        println!("Hi {}! this is how we create a commandline", name) //cargo run hello
    }
    else if command == "status"
    {
        println!("Status level: {}", status);
    }
    else
    {
        println!("That is not a valid command.")
    }
}