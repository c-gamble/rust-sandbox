//enums are types which have a few definite values

enum Movement {
    //variants
    Up,
    Down,
    Left,
    Right
}

fn move_player(m: Movement) {
    //perform action depending on Movement
    match m { //similar to a switch
        Movement::Up => println!("Player moved up."),
        Movement::Down => println!("Player moved down."),
        Movement::Left => println!("Player moved left."),
        Movement::Right => println!("Player moved right.")
    } 
}

pub fn run() {
    let player_1 = Movement::Left;
    let player_2 = Movement::Right;
    let player_3 = Movement::Down;
    let player_4 = Movement::Up;

    move_player(player_1);
    move_player(player_2);
    move_player(player_3);
    move_player(player_4);
}

