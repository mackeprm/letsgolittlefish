use std::collections::HashMap;

use clap::Parser;
use rand::{Rng, rngs::ThreadRng};

const WINNERS: [&str; 2] = ["fish", "fishers"];
const DIE_SIDES: [&str; 6] = ["red", "green", "pink", "orange", "yellow", "blue"];

#[derive(Parser)]
#[command(name = "Letsgolittlefish")]
#[command(version = "1.0")]
#[command(about = "Simulates runs of the board game 'Let's go little fish'.", long_about = None)]
struct Cli {
    #[arg(long, default_value_t = 5)]
    fish_tiles: u16,
    #[arg(long, default_value_t = 5)]
    boat_tiles: u16,
    #[arg(long, default_value_t = 10000)]
    n: u16,
}

//TODO struct GameBoard

struct Piece<'a> {
    position: u16,
    state: &'a str,
}

fn dice_roll(rng: &mut ThreadRng) -> &str {
    let random_index = rng.random_range(0..DIE_SIDES.len());
    return DIE_SIDES[random_index];
}

fn check_win_condition(
    boat: &Piece,
    blue_fish: &Piece,
    orange_fish: &Piece,
    pink_fish: &Piece,
    yellow_fish: &Piece,
) -> bool {
    if boat.position == 0 {
        return true;
    }
    if blue_fish.state != "active"
        && orange_fish.state != "active"
        && pink_fish.state != "active"
        && yellow_fish.state != "active" {
        return true;
    }
    return false;
}

fn run_game(cli: &Cli) -> &str {
    /*Game board setup:
        River:
            [0] is Sea
            [fish-tiles + 1] is fish init position
            [fish-tiles + 1 + boat-tiles + 1] = boat position
    */
    let mut boat = Piece {
        position: cli.fish_tiles + cli.boat_tiles + 2,
        state: "active",
    };
    //TODO optimize: hashmap color -> Piece
    let mut yellow_fish = Piece {
        position: cli.fish_tiles + 1,
        state: "active",
    };
    let mut blue_fish = Piece {
        position: cli.fish_tiles + 1,
        state: "active",
    };
    let mut pink_fish = Piece {
        position: cli.fish_tiles + 1,
        state: "active",
    };
    let mut orange_fish = Piece {
        position: cli.fish_tiles + 1,
        state: "active",
    };
    //loop:
    while !check_win_condition(&boat, &blue_fish, &orange_fish, &pink_fish, &yellow_fish) {
        // Dice Roll:
        let mut rng = rand::rng();
        let current_dice_roll = dice_roll(&mut rng);
        match current_dice_roll {
            "red" => boat.position = boat.position - 1,
            "green" => boat.position = boat.position - 1,
            "blue" => blue_fish.position = update_fish_position(&blue_fish), //TODO only if fish is active
            "orange" => orange_fish.position = update_fish_position(&orange_fish), //TODO only if fish is active
            "pink" => pink_fish.position = update_fish_position(&pink_fish), //TODO only if fish is active
            "yellow" => yellow_fish.position = update_fish_position(&yellow_fish), //TODO only if fish is active
            _ => panic!("this is a terrible mistake"),
        };
        update_fish_state(&mut blue_fish, &boat);
        update_fish_state(&mut orange_fish, &boat);
        update_fish_state(&mut pink_fish, &boat);
        update_fish_state(&mut yellow_fish, &boat);
        //print_game_board(&boat, &blue_fish, &orange_fish, &pink_fish, &yellow_fish);
    }
    let winner = determine_winner(&blue_fish, &orange_fish, &pink_fish, &yellow_fish);
    //println!("{}", winner);
    return winner;
}

//TODO move this to a pure function.
fn update_fish_state<'a>(fish: &'a mut Piece<'_>, boat: &Piece<'_>) {
    if fish.position == 0 {
        fish.state = "free";
    } else if boat.position <= fish.position {
        fish.state = "captured";
    }
}

fn update_fish_position(fish: &Piece<'_>) -> u16 {
    if fish.state != "active" {
        //TODO this is not the original logic: in the original,
        // if the fish is already "free": move another fish
        // if the fish is already "captured": move the boat.
        return fish.position;
    } else {
        return fish.position - 1;
    }
}

fn determine_winner<'a>(
    blue_fish: &Piece,
    orange_fish: &Piece,
    pink_fish: &Piece,
    yellow_fish: &Piece,
) -> &'a str {
    let mut num_fish_escaped: u8 = 0;
    if blue_fish.state == "free" {
        num_fish_escaped += 1;
    }
    if orange_fish.state == "free" {
        num_fish_escaped += 1;
    }
    if pink_fish.state == "free" {
        num_fish_escaped += 1;
    }
    if yellow_fish.state == "free" {
        num_fish_escaped += 1;
    }

    if num_fish_escaped >= 2 {
        return WINNERS[0];
    } else {
        return WINNERS[1];
    }
}

/*fn print_game_board(
    boat: &Piece,
    blue_fish: &Piece,
    orange_fish: &Piece,
    pink_fish: &Piece,
    yellow_fish: &Piece,
) {
    println!(
        "Current Game State: boat{},b{},o{},p{},y{}",
        boat.position,
        blue_fish.position,
        orange_fish.position,
        pink_fish.position,
        yellow_fish.position
    );
}*/

fn main() {
    let cli = Cli::parse();
    println!("fish_tiles: {:?}", cli.fish_tiles);
    println!("boat_tiles: {:?}", cli.boat_tiles);
    println!("iterations: {:?}", cli.n);

    let mut counts: HashMap<&str, usize> = HashMap::new();

    for _ in 0..cli.n {
        let result = run_game(&cli);
        *counts.entry(result).or_insert(0) += 1;
    }

    // Print the results
    for (string, count) in counts {
        println!("{} was chosen {} times", string, count);
    }
}
