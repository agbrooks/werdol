use core::convert::TryInto;
use std::cmp::min;

use bevy::{prelude::*, reflect::Enum};
use rand::{distributions::Uniform, Rng};

extern crate derive_more;
#[macro_use]
extern crate lazy_static;
use derive_more::TryInto;

lazy_static! {
    static ref TILE_STYLE: Style = Style {
        size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
        margin: UiRect::all(Val::Percent(1.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    static ref TEXT_STYLE: TextStyle = TextStyle {
        font_size: 120.0,
        color: Color::WHITE,
        ..default()
    };
}

// "Tag component" to figure out where in the NodeBundle tree the game board
// entity actually lives
#[derive(Component)]
struct GameBoard;

/// A tile on the Werdol board
#[derive(Component, Clone, Copy, TryInto, Debug)]
#[try_into(owned, ref, ref_mut)]
enum Tile {
    /// Correct letter in the correct position
    Correct(char),
    /// Correct letter, wrong position
    Misplaced(char),
    /// Letter not in the target word whatsoever
    Missing(char),
    /// Has a letter, but word isn't submitted yet, so we can't reveal how
    /// correct it is yet.
    Unconfirmed(char),
    /// A tile without any guessed letter
    Blank
}

impl Default for Tile {
    fn default() -> Self { Self::Blank }
}

impl Tile {
    // Forget a proposed character for this tile.
    fn delete(&mut self) {
        *self = Self::Blank
    }

    // Propose a character for a tile, but don't confirm whether or not it was guessed
    // correctly.
    fn input(&mut self, guess: char) {
        *self = Self::Unconfirmed(guess)
    }

    // Check a character against a word.
    fn check(&mut self, word: &[char], actual: char) {
        *self = match *self {
            Self::Unconfirmed(guess) if guess == actual                  => Self::Correct(guess),
            Self::Unconfirmed(guess) if word.iter().any(|c| *c == guess) => Self::Misplaced(guess),
            Self::Unconfirmed(guess)                                     => Self::Missing(guess),
            _ => self.clone()
        }
    }

    // Ask a tile how it should be displayed
    fn color(&self) -> Color {
        match self {
            Self::Correct(_)   => Color::rgb(0.0, 0.5, 0.0),
            Self::Misplaced(_) => Color::rgb(0.5, 0.5, 0.0),
            Self::Missing(_)   => Color::rgb(0.25, 0.25, 0.25),
            _                  => Color::GRAY
        }
    }

    fn is_correct(&self) -> bool {
        match self {
            Self::Correct(_) => true,
            _ => false
        }
    }

    // Get the character that should be displayed on a tile (if it exists)
    fn get_chr(&self) -> Option<char> {
        (*self).try_into().ok()
    }

    fn text(&self) -> String {
        self
            .clone()
            .try_into()
            .map_or("".to_string(), |c: char| c.to_string())
    }

    fn draw_as_child_of(&self, parent: &mut ChildBuilder<'_, '_, '_>, asset_server: &Res<AssetServer>) {
        let mut txt_style = TEXT_STYLE.clone();
        txt_style.font = asset_server.load("fonts/FiraSans-Bold.ttf");
        parent
            .spawn(
                ButtonBundle {
                    style: TILE_STYLE.clone(),
                    background_color: self.color().into(),
                    ..default()
                }
            )
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    self.text(),
                    txt_style.clone()
                ));
            });
    }
}

#[derive(Resource, Debug, Clone, Copy)]
struct Game {
    tiles: [[Tile; 5]; 5],
    answer: [char; 5],
    row: usize, // 0..=4
    col: usize, // 0..=4
    done: bool
}

impl Game {
    pub fn new(answer: [char; 5]) -> Game {
        Game {
            tiles: [[Tile::Blank; 5]; 5],
            row: 0,
            col: 0,
            // I am regretting not using Vecs
            answer: answer.iter().map(|x| x.clone().to_ascii_uppercase()).collect::<Vec<_>>().try_into().unwrap(),
            done: false
        }
    }

    /// Nuke the game w/ a new word and start over
    pub fn reset(&mut self, answer: [char; 5]) {
        *self = Game::new(answer)
    }

    /// Submit an entire row, return indicates if successful
    /// (may fail due to being an incomplete row, stuff like that)
    pub fn submit_row(&mut self) -> bool {
        // Guess is incomplete, cannot submit.
        if self.col != 4 {
            return false
        }
        let r = min(self.row as usize, 4);
        let mut i = 0;
        for tile in &mut self.tiles[r] {
            tile.check(&self.answer, self.answer[i]);
            i += 1;
        }
        if self.row == 4 || self.won() {
            self.done = true;
        }

        if self.row < 4 {
            self.row += 1;
        }
        self.col = 0;
        true
    }

    // Fill in a character, without necessarily submitting it as a guess
    pub fn submit_char(&mut self, chr: char) {
        if self.done {
            return
        }
        self.tiles[self.row][self.col].input(chr.to_ascii_uppercase());
        self.col = min(self.col + 1, 4)
    }

    pub fn delete_char(&mut self) {
        self.tiles[self.row][self.col].delete();
        if self.done || self.col == 0 {
            return
        }
        self.col -= 1;
        self.tiles[self.row][self.col].delete();
    }

    pub fn won(&self) -> bool {
        self.tiles[self.row].iter().all(|x| x.is_correct())
    }

    pub fn lost(&self) -> bool {
        self.done && !self.won()
    }
}


fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Handle keyboard input
fn handle_input(kbd_input: Res<Input<KeyCode>>, mut game: ResMut<Game>) {
    if game.done { return }
    for code in kbd_input.get_just_pressed() {
        match code {
            KeyCode::Return => {game.submit_row();},
            KeyCode::Back   => {game.delete_char();},
            k if *k >= KeyCode::A && *k <= KeyCode::Z => {
                // HACK: Yes, we're using reflection. Yes, it's gross.
                game.submit_char(k.variant_name().chars().next().expect("unnamed keycode variant") as char);
            },
            KeyCode::Escape => {game.reset(pick_word());},
            _ => ()
        }   
    }
}

// Create a placeholder nodebundle for the game board
fn spawn_game_board(mut commands: Commands) {
    commands
      .spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        },
        GameBoard
    ));
}

fn redraw_tiles(mut commands: Commands, game: Res<Game>, asset_server: Res<AssetServer>, query: Query<Entity, With<GameBoard>>) {
    // TODO: Should probably handle the assetserver stuff here, no need to repeat font stuff
    for e in query.iter() {
        let mut entity_cmds = commands.entity(e);
        entity_cmds.despawn_descendants();
        entity_cmds.with_children(|parent| {
            for row in &game.tiles {
                // Bundle containing each row of the werdol board
                parent.spawn(
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_wrap: FlexWrap::Wrap,
                            size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    }
                )
                    .with_children(|parent| {
                        parent.spawn(
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    ..default()
                                },
                                background_color: Color::BLACK.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                for tile in row {
                                    tile.draw_as_child_of(parent, &asset_server);
                                }
                            });
                    });
            }
        });
    }
}

fn pick_word() -> [char; 5] {
    rand::thread_rng()
        .sample_iter(Uniform::new(0u8, 26))
        .take(5)
        .map(|offset| char::from('A' as u8 + offset))
        .collect::<Vec<_>>()
        .try_into()
        .expect("unable to build random word")
}

fn main() {
    let word = pick_word();
    // TODO: add troll mode and a "real" wordlist to use
    println!("The answer is {:?} (you _cheater_)", word);
    let mut game = Game::new(word);
    game.submit_row();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK.into()))
        .insert_resource(game)
        .add_startup_system(camera_setup)
        .add_startup_system(spawn_game_board)
        .add_system(redraw_tiles)
        .add_system(handle_input)
        //.add_system(bevy::window::close_on_esc)
        .run();
}
