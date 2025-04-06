use std::collections::HashMap;

use clap::Parser;
use rand::{Rng, rngs::ThreadRng};

//TODO move to ints + constants
const WINNERS: [&str; 3] = ["fish", "fishers", "tie"];
//TODO move to ints + constants
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

fn dice_roll(rng: &mut ThreadRng) -> &str {
    let random_index = rng.random_range(0..DIE_SIDES.len());
    return DIE_SIDES[random_index];
}

struct Piece<'a> {
    position: u16,
    state: &'a str,
}

fn check_win_condition(boat: &Piece, fish: &[Piece]) -> bool {
    if boat.position == 0 {
        return true;
    }
    return fish.iter().any(|piece| piece.state == "active") == false;
}

fn determine_winner<'a>(fish: &[Piece]) -> &'a str {
    let num_fish_escaped = fish.iter().filter(|&fish| fish.state == "free").count();
    if num_fish_escaped > 2 {
        return WINNERS[0];
    } else if num_fish_escaped == 2 {
        return WINNERS[2];
    } else {
        return WINNERS[1];
    }
}

//TODO move this to a pure function.
fn update_fish_state<'a>(fish: &'a mut Piece<'_>, boat: &Piece<'_>) {
    if fish.position == 0 {
        fish.state = "free";
    } else if boat.position <= fish.position {
        fish.state = "captured";
    }
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

    /*
     Fixed size array, because HashMap would allocate on the heap, and i don't want that.
     Mapping is:
        [0] Blue
        [1] Orange
        [2] Pink
        [3] Yellow
    */
    let fish_start = cli.fish_tiles + 1;
    let mut fish = [
        Piece {
            position: fish_start,
            state: "active",
        },
        Piece {
            position: fish_start,
            state: "active",
        },
        Piece {
            position: fish_start,
            state: "active",
        },
        Piece {
            position: fish_start,
            state: "active",
        },
    ];
    //loop:
    while !check_win_condition(&boat, &fish) {
        // Dice Roll:
        let mut rng = rand::rng();
        let current_dice_roll = dice_roll(&mut rng);
        if current_dice_roll == "red" || current_dice_roll == "green" {
            boat.position -= 1
        } else {
            let fish_index: usize = match current_dice_roll {
                "blue" => 0,
                "orange" => 1,
                "pink" => 2,
                "yellow" => 3,
                _ => panic!("this is a terrible mistake"),
            };
            //TODO filter + map?
            let selected_fish_state = fish[fish_index].state;
            if selected_fish_state == "active" {
                fish[fish_index].position -= 1;
            } else if selected_fish_state == "captured" {
                boat.position -= 1
            } else if selected_fish_state == "free" {
                update_by_strategy(&mut fish);
            } else {
                panic!("this is a terrible mistake");
            }
        }
        for current_fish in &mut fish {
            update_fish_state(current_fish, &boat);
        }
        //print_game_board(&boat, &fish[0], &fish[1], &fish[2], &fish[3]);
    }
    let winner = determine_winner(&fish);
    //println!("{}", winner);
    return winner;
}

fn move_next_active_fish(fish: &mut [Piece<'_>; 4]) -> () {
    // Move the next active fish we found:
    for current_fish in fish {
        if current_fish.state == "active" {
            current_fish.position -= 1;
            break;
        }
    }
}

// Find the active piece with the lowest position value
fn move_fish_nearest_sea(fish: &mut [Piece<'_>; 4]) -> () {
    let maybe_min_piece_index = fish
        .iter()
        .enumerate()
        .filter(|(_, piece)| piece.state == "active")
        .min_by_key(|(_, piece)| piece.position)
        .map(|(index, _)| index);

    if let Some(i) = maybe_min_piece_index {
        fish[i].position -= 1;
    }
}

fn move_fish_farthest_from_sea(fish: &mut [Piece<'_>; 4]) -> () {
    let maybe_min_piece_index = fish
        .iter()
        .enumerate()
        .filter(|(_, piece)| piece.state == "active")
        .max_by_key(|(_, piece)| piece.position)
        .map(|(index, _)| index);

    if let Some(i) = maybe_min_piece_index {
        fish[i].position -= 1;
    }
}

//TODO move to pure function?
//TODO switch strategy by CLI
fn update_by_strategy(fish: &mut [Piece<'_>; 4]) -> () {
    //move_next_active_fish(fish);
    //move_fish_nearest_sea(fish);
    move_fish_farthest_from_sea(fish);
}

//TODO track number of rounds per game.
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
