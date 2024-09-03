#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::{println, read_private_input, write_output};

#[derive(Default)]
struct Player {
    x: i32,
    y: i32,
    health: u32,
    ammo: u32,
}

#[derive(Default)]
struct GameState {
    player: Player,
    enemies_killed: u32,
}

#[nexus_rt::main]
fn main() {
    let input: Result<(i32, i32, bool), _> = read_private_input();

    let mut game_state = GameState::default();
    game_state.player.health = 100;
    game_state.player.ammo = 50;

    if let Ok((move_x, move_y, shoot)) = input {
        // Move player
        game_state.player.x += move_x;
        game_state.player.y += move_y;

        // Shoot if requested
        if shoot && game_state.player.ammo > 0 {
            game_state.player.ammo -= 1;
            game_state.enemies_killed += 1;
        }

        println!("Player moved to ({}, {})", game_state.player.x, game_state.player.y);
        println!("Enemies killed: {}", game_state.enemies_killed);
        println!("Ammo left: {}", game_state.player.ammo);
    } else {
        println!("No input provided, game state unchanged.");
    }

    // Output the final game state
    let output = (
        game_state.player.x,
        game_state.player.y,
        game_state.player.health,
        game_state.player.ammo,
        game_state.enemies_killed,
    );
    write_output(&output);
}