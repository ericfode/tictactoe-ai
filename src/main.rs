use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use tictactoe_ai_rust::game::{TicTacToe};
use tictactoe_ai_rust::bevy::{setup, next_move, CurrentMove, PieceSprites, AIs};

fn main() {
    let mut game = TicTacToe::new();
    game.make_move(0, 0).unwrap();
    game.make_move(0, 1).unwrap();
    game.make_move(1, 1).unwrap();
    game.make_move(2, 2).unwrap();
    game.make_move(2, 0).unwrap();
    game.pretty_print();
    game.make_move(2, 1).unwrap();
    game.make_move(1, 0).unwrap();
    game.pretty_print();
    if game.is_game_over() {
        println!("Game over!");
    } else {
        println!("Game is not over yet");
    }

    let mut game = TicTacToe::new();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window{
                    title: "tic tac toe".into(),
                    resolution: (1200.,1200.).into(),
                    ..default()
                }),
                ..default()
            }
        ))
        .init_resource::<TicTacToe>()
        .init_resource::<CurrentMove>()
        .init_resource::<PieceSprites>()
        .init_resource::<AIs>()
        .add_startup_system(setup)
        .add_system(next_move)
        .run();

}