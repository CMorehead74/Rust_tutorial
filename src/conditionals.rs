//Conditionals - used to check the condition of something and act of the result

pub fn run()
{
    let age: u8 = 18;
    let mut check_id: bool = false;
    //let knows_person_of_age = true;

    //if/else
    if check_id
    {
        check_id = true;
    }

    if age >= 21 && check_id //|| knows_person_of_age
    {
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && check_id
    {
        println!("Bartender: Sorry, you have to be 21!");
    }
    else
    {
        println!("Bartender: Id please.");
    }

    let is_of_age = if age >= 21 { true } else { false };

    println!("Is of age: {}", is_of_age);

}