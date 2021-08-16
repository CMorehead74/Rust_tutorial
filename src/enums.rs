//enums are type which have a few definite values

enum Movement
{
    Up, Down, Left, Right
}

fn move_avatar(m: Movement)
{
    //perform action depending on input
    match m //match is a switch
    {
        Movement::Up => println!("Player moves upward."),
        Movement::Down => println!("Player moves downward."),
        Movement::Left => println!("Player moves left."),
        Movement::Right => println!("Player moves right.")
    }
}

pub fn run()
{
    let leftkey = Movement::Left;
    let rightkey = Movement::Right;
    let upkey = Movement::Up;
    let downkey = Movement::Down;

    move_avatar(upkey);
    move_avatar(rightkey);
    move_avatar(leftkey);
    move_avatar(downkey);

}