<<<<<<< HEAD
use core::convert::TryInto;
use std::cmp::min;

use bevy::{prelude::*, reflect::Enum};
use rand::{distributions::Uniform, Rng};
=======
use std::cmp::{min, max};

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use core::convert::TryInto;

use rand::{distributions::Alphanumeric, Rng};
>>>>>>> 37480eb (Get a wordle board drawing (badly))

extern crate derive_more;
#[macro_use]
extern crate lazy_static;
use derive_more::TryInto;

lazy_static! {
    static ref TILE_STYLE: Style = Style {
        size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
<<<<<<< HEAD
        margin: UiRect::all(Val::Percent(1.0)),
=======
        //size: Size::AUTO,
        margin: UiRect::all(Val::Percent(4.0)),
>>>>>>> 37480eb (Get a wordle board drawing (badly))
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

<<<<<<< HEAD
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppState {
    /// Game is ongoing
    Playing,
    /// Game completed (won)
    Won,
    /// Game completed (lost)
    Lost
}

// "Tag component" to figure out where in the NodeBundle tree the game board
// entity actually lives
#[derive(Component)]
struct GameBoard;

// "Tag component" to identify part of NodeBundle tree containing a
// win/loss notification
#[derive(Component)]
struct Notification;

/// A tile on the Werdol board
#[derive(Component, Clone, Copy, TryInto, Debug, Eq, PartialEq, Hash)]
=======
// stupid, distinctive tag to use
#[derive(Component)]
struct Grimbo {}

/// A tile on the Werdol board
#[derive(Component, Clone, Copy, TryInto, Debug)]
>>>>>>> 37480eb (Get a wordle board drawing (badly))
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

<<<<<<< HEAD
/// A single tile on the Werdol board.
impl Tile {
    // Forget a proposed character for this tile.
=======
impl Tile {
    // Forget a prposed character for this tile.
>>>>>>> 37480eb (Get a wordle board drawing (badly))
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
<<<<<<< HEAD
            Self::Correct(_)   => Color::rgb(0.0, 0.5, 0.0),
            Self::Misplaced(_) => Color::rgb(0.5, 0.5, 0.0),
            Self::Missing(_)   => Color::rgb(0.25, 0.25, 0.25),
=======
            Self::Correct(_)   => Color::GREEN,
            Self::Misplaced(_) => Color::YELLOW,
>>>>>>> 37480eb (Get a wordle board drawing (badly))
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
<<<<<<< HEAD

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
=======
}

// TODO: Also need a nice interface to input a word...

// TODO: Should have a 'resource' to track the game state... like what the target word is
// TODO: Note that drawing logic is totally independent of the actual components being tracked
// Real game example:
// https://github.com/bevyengine/bevy/blob/v0.8.1/examples/games/breakout.rs



#[derive(Resource, Debug, Clone, Copy)]
struct Grid {
>>>>>>> 37480eb (Get a wordle board drawing (badly))
    tiles: [[Tile; 5]; 5],
    answer: [char; 5],
    row: usize, // 0..=4
    col: usize, // 0..=4
    done: bool
}

<<<<<<< HEAD
impl Game {
    pub fn new(answer: [char; 5]) -> Game {
        Game {
            tiles: [[Tile::Blank; 5]; 5],
            row: 0,
            col: 0,
            // I am regretting not using Vecs
=======
impl Grid {
    pub fn new(answer: [char; 5]) -> Grid {
        Grid {
            tiles: [[Tile::Blank; 5]; 5],
            row: 0,
            col: 0,
            // holy dammit christmas
>>>>>>> 37480eb (Get a wordle board drawing (badly))
            answer: answer.iter().map(|x| x.clone().to_ascii_uppercase()).collect::<Vec<_>>().try_into().unwrap(),
            done: false
        }
    }

<<<<<<< HEAD
    /// Nuke the game w/ a new word and start over
    pub fn reset(&mut self, answer: [char; 5]) {
        *self = Game::new(answer)
=======
    /// Nuke the grid w/ a new word and start over
    pub fn reset(&mut self, answer: [char; 5]) {
        *self = Grid::new(answer)
>>>>>>> 37480eb (Get a wordle board drawing (badly))
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
<<<<<<< HEAD
        self.col = 0;
=======
>>>>>>> 37480eb (Get a wordle board drawing (badly))
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

<<<<<<< HEAD
    // Retract a proposed character, so long as it's still unsubmitted
    pub fn delete_char(&mut self) {
        self.tiles[self.row][self.col].delete();
        if self.done || self.col == 0 {
            return
        }
        self.col -= 1;
        self.tiles[self.row][self.col].delete();
    }

    pub fn won(&self) -> bool {
        // FIXME: Always seems to return false, huh
=======
    pub fn delete_char(&mut self) {
        if self.done || self.col == 0 {
            return
        }
        self.tiles[self.row][self.col].delete();
        self.col -= 1
    }

    pub fn won(&self) -> bool {
>>>>>>> 37480eb (Get a wordle board drawing (badly))
        self.tiles[self.row].iter().all(|x| x.is_correct())
    }

    pub fn lost(&self) -> bool {
        self.done && !self.won()
    }
<<<<<<< HEAD
=======

    /// Last row typed, extended w/ zero bytes if incomplete
    fn current_row_chars(&self) -> [char; 5] {
        let r = min(self.row as usize, 4);
        self.tiles[r]
            .iter()
            .map(|x| x.get_chr().unwrap_or_default())
            .collect::<Vec<_>>().try_into().expect("oh no")
    }

>>>>>>> 37480eb (Get a wordle board drawing (badly))
}


fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

<<<<<<< HEAD
// Handle keyboard input
fn handle_game_input(kbd_input: Res<Input<KeyCode>>, mut game: ResMut<Game>, mut state: ResMut<State<AppState>>) {
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
    if game.done {
        if game.won() {
            state.overwrite_replace(AppState::Won);
        } else {
            state.overwrite_replace(AppState::Lost);
        }
    }
}

fn restart_on_keypress(kbd_input: Res<Input<KeyCode>>, mut game: ResMut<Game>, mut state: ResMut<State<AppState>>) {
    for _ in kbd_input.get_just_pressed() {
        // HACK: For reasons I do not understand, the enter keypress that was used for the previous state will
        // be sent to this function.
        //
        // In this function, we reset the game on the first received, then switch game states on the second.
        // I should revisit this.
        if game.done {
            game.reset(pick_word());
        } else {
            state.overwrite_replace(AppState::Playing);
        }
    }
}

// Create a placeholder nodebundle for the game board
fn spawn_game_board(mut commands: Commands) {
=======
// fn add_tile(mut commands: Commands) {
//    commands
//         .spawn(
//             ButtonBundle {
//                 style: TILE_STYLE.clone(),
//                 background_color: Tile::Blank.color().into(),
//                 //background_color: Tile::Correct(b'A').color().into(),
//                 ..default()
//             }
//         );
// }

fn add_tile(mut commands: Commands) {
   commands
        .spawn(
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            })
        .with_children(|parent| {
            parent.spawn(
            ButtonBundle {
                style: TILE_STYLE.clone(),
                background_color: Tile::Correct(b'A' as char).color().into(),
                ..default()
            });  
        });
}

// Could also have query: Query<&Tile, With<Other>> for things
// that are spawned together (like, all people with a Name or something)
fn look_at_tiles(mut query: Query<&mut Tile>) {
    for tile in query.iter() {
        println!("TILE: {:?}", tile);
    }
}

fn look_at_grid(mut grid: Res<Grid>) {
    println!("GRID: {:?}", grid);
}

// TODO left off here: want to render and de-render all tiles when certain events occur.
fn spawn_grid(mut commands: Commands, grid: Res<Grid>, asset_server: Res<AssetServer>) {
    
    let mut txt_style = TEXT_STYLE.clone();
    txt_style.font = asset_server.load("fonts/FiraSans-Bold.ttf");

>>>>>>> 37480eb (Get a wordle board drawing (badly))
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
<<<<<<< HEAD
        },
        GameBoard
    ));
}

// TODO should find a nice way to avoid having to explicitly pass the commands/asset server
fn make_notification(msg: impl Into<String>, color: Color, mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut txt_style = TEXT_STYLE.clone();
    txt_style.font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: color.into(),
                ..default()
            },
            Notification
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        msg, txt_style.clone()
                    ),
                    TextSection::new(
                        "(press any key)", txt_style.clone()
                    )
                ]))
            );
        });
}

fn clear_notification(mut commands: Commands, mut query: Query<Entity, With<Notification>>) {
    for e in query.iter_mut() {
        commands.entity(e).despawn_descendants();
        commands.entity(e).despawn();
    }
}

fn notify_won(mut commands: Commands, asset_server: Res<AssetServer>) {
    make_notification("You won!\n", Color::DARK_GREEN, commands, asset_server)
}

fn notify_lost(mut commands: Commands, asset_server: Res<AssetServer>, game: Res<Game>) {
    let s: String = game.answer.iter().collect();
    make_notification(format!("You lose, doofus!\nThe word was '{}'.\n", s), Color::CRIMSON, commands, asset_server)
}

fn hide_board(mut commands: Commands, query: Query<Entity, With<GameBoard>>) {
    for e in query.iter() {
        commands.entity(e).despawn_descendants();
    }
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
        .add_state(AppState::Playing)
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(clear_notification)
                .with_system(redraw_tiles)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(handle_game_input)
                .with_system(redraw_tiles)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Playing)
                .with_system(hide_board)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Won).with_system(notify_won)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Won).with_system(restart_on_keypress)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Lost).with_system(notify_lost)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Lost).with_system(restart_on_keypress)
        )
=======
        }
    ))
    .with_children(|parent| {
        for row in &grid.tiles {
            // Bundle containing each row of the werdol board
            parent.spawn((
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
            ))
            .with_children(|parent| {
                 parent.spawn((
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
                     }))
                .with_children(|parent| {
                    for tile in row {
                        parent.spawn((
                            ButtonBundle {
                                style: TILE_STYLE.clone(),
                                background_color: tile.color().into(),
                                ..default()
                            })
                        )
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                tile.text(),
                                txt_style.clone()
                            ));
                        });
                    }
                });
             });
        }
    });
}

// TODO
// fn despawn_grid(mut commands: Commands)

fn pick_word() -> [char; 5] {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect::<Vec<_>>()
        .try_into()
        .expect("random word generation failed (for some reason)")
}

fn main() {
    let mut grid = Grid::new(pick_word());
    for _ in (0..5) {
        grid.submit_char('c');
    }
    grid.submit_row();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::GREEN.into()))
        .insert_resource(grid)
        .add_startup_system(camera_setup)
        .add_startup_system(spawn_grid)
        .add_system(look_at_tiles)
        .add_system(bevy::window::close_on_esc)
        //.add_startup_system(add_tile)
        //.add_system(look_at_grid)
>>>>>>> 37480eb (Get a wordle board drawing (badly))
        .run();
}
