use bevy::{prelude::*, input::keyboard, transform::commands};

use crate::game::{TicTacToe, Player, TicTacToeAI, MinimaxAI, RandomAI};

const SPRITE_SIZE: Vec3 = Vec3::new(300.0, 300.0, 0.0);

const MOVE_LIST: [(usize, usize); 9] = [
    (0, 0), (1, 0), (2, 0),
    (0, 1), (1, 1), (2, 1),
    (0, 2), (1, 2), (2, 2),
];

#[derive(Component)]
pub struct AMove;

#[derive(Resource, Default)]
pub struct PieceSprites {
    x_sprite: Handle<Image>,
    y_sprite: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct AIs {
    pub x_ai: MinimaxAI,
    pub y_ai: RandomAI,
}

impl AIs {
    pub fn make_move(&self, game: &TicTacToe) -> (usize, usize) {
        match game.get_current_player() {
            Player::X => self.x_ai.get_move(game),
            Player::O => self.y_ai.get_move(game),
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentMove(usize);

impl Player {
    pub fn to_sprite(&self, peice_sprites:  Res<PieceSprites>) -> Handle<Image> {
        match self {
            Player::X => peice_sprites.x_sprite.clone(),
            Player::O => peice_sprites.y_sprite.clone(),
        }
    }
}

pub fn next_move(
    mut commands:  Commands,
    keyboard: Res<Input<KeyCode>>,
    peice_sprites: Res<PieceSprites>,
    ais: Res<AIs>,
    mut tictactoe: ResMut<TicTacToe>,
    mut current_move: ResMut<CurrentMove>,
    mut all_moves: Query<(Entity, &AMove)>,
) {
    let current_move_value = current_move.0;



    if keyboard.just_pressed(KeyCode::Space) {
            if tictactoe.is_game_over() {
                tictactoe.reset();
                // Despawn all moves
                for (entity, _) in all_moves.iter_mut() {
                    commands.entity(entity).despawn();
                }
                println!("reset");
                return;
            }
            let next_move = ais.make_move(&tictactoe);
            let result =tictactoe.make_move(next_move.0, next_move.1);
            if result.is_err() {
                println!("Invalid move");
                return;
            }
            let move_x =  next_move.0 as f32 - 1.0;
            let move_y =  (2.0 + -(next_move.1 as f32)) - 1.0;
            let move_offset_x = move_x * (SPRITE_SIZE.x + 100.0);
            let move_offset_y = move_y * (SPRITE_SIZE.y + 100.0);
            
            commands.spawn((SpriteBundle {
                texture: tictactoe.get_current_player().to_sprite(peice_sprites),
                transform: Transform::from_translation(Vec3::new(
                    move_offset_x,
                    move_offset_y,
                    1.0,
                )),
                ..default()
            },
            AMove)
        );
            
            current_move.0 = current_move_value + 1;
            tictactoe.pretty_print();
    }
}


pub fn setup(mut commands: Commands,
            mut current_move: ResMut<CurrentMove>,
            mut piece_sprites: ResMut<PieceSprites>,
            asset_server: Res<AssetServer>) {
    // Create camera
    commands.spawn(Camera2dBundle::default());

    // Load board and piece sprites
    let board_sprite: Handle<Image> = asset_server.load("board.png");
    piece_sprites.x_sprite= asset_server.load("x.png");
    piece_sprites.y_sprite = asset_server.load("o.png");
    current_move.0 = 0;

    commands.spawn((SpriteBundle {
        texture: board_sprite,
        ..default()
    },
    ));
}
