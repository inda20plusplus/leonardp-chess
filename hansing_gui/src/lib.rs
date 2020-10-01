use ggez::conf::{ WindowSetup};
use ggez::event::MouseButton;
use ggez::graphics::DrawParam;
use ggez::{event, graphics::*, Context, GameResult};
use std::{env, path};

extern crate chess_engine;
use chess_engine::color::Color as PieceColor;
use chess_engine::piece::PieceKind;
use chess_engine::Game;
use chess_engine::Position;

mod texturepack;
use texturepack::Texturepack;
//mod assets;
mod interface;

const BOARD_SIZE: f32 = 800.0;
const BOARD_DIM: usize = 8;
const BORDER: f32 = 10.0;

#[derive(PartialEq, Copy, Clone)]
pub struct Tile {
    rank: usize,
    file: usize,
}
impl Tile {
    fn get_index(&self) -> usize {
        self.rank * 8 + self.file
    }
    fn from_index(index: usize) -> Self {
        Self {
            rank: index / 8,
            file: index % 8,
        }
    }
    pub fn from_mouse_pos(x: f32, y: f32) -> Option<Self> {
        let rank = (y - BORDER) / (BOARD_SIZE / BOARD_DIM as f32);
        let file = (x - BORDER) / (BOARD_SIZE / BOARD_DIM as f32);
        if rank >= 0.0 && rank < BOARD_DIM as f32 && file >= 0.0 && file < BOARD_DIM as f32 {
            Some(Self {
                rank: rank as usize,
                file: file as usize,
            })
        } else {
            None
        }
    }
    fn get_color(&self) -> Color {
        if (self.rank + self.file) % 2 == 0 {
            Color::from_rgb(240, 220, 200)
        } else {
            Color::from_rgb(40, 80, 40)
        }
    }
    fn to_pos(&self) -> Position{
        Position{
            y: self.rank,
            x: self.file,
        }
    }
}
struct MainState {
    selected_tile: Option<Tile>,
    game: Game,
    display_data: Vec<Option<(PieceColor, PieceKind)>>,
    update_visuals: bool,
    texturepack: Texturepack, 
}

impl MainState {
    fn new() -> Self {
        Self {
            selected_tile: None,
            game: Game::new_standard_game(), 
            update_visuals: true,
            display_data: Vec::new(),
            texturepack: Texturepack::new_placeholder(),
        }
    }

    fn render(&mut self, c: &mut Context) {
        clear(c, Color::from_rgb(20, 20, 60));
        for i in 0..(BOARD_DIM * BOARD_DIM) {
            let tile = Tile::from_index(i);
            let color;
            if Some(tile) == self.selected_tile {
                color = Color::from_rgb(200, 180, 30);
            } else {
                color = tile.get_color();
            }

            let rect = Rect::new(
                (tile.file as f32) * BOARD_SIZE / (BOARD_DIM as f32) + BORDER,
                (tile.rank as f32) * BOARD_SIZE / (BOARD_DIM as f32) + BORDER,
                BOARD_SIZE / (BOARD_DIM as f32),
                BOARD_SIZE / (BOARD_DIM as f32),
            );

            let tile_mesh = Mesh::new_rectangle(c, DrawMode::fill(), rect, color);
            if let Ok(tile_mesh) = tile_mesh {
                let res = draw(c, &tile_mesh, DrawParam::default());
                if let Err(res) = res{
                    println!("{:?}",res);
                }
                assert!(self.display_data.len() > tile.get_index());

                if let Some((pc, pk)) = &self.display_data[tile.get_index()] {
                    let piece_mesh: Image = self
                        .texturepack
                        .texture_from_index(
                            if *pc == PieceColor::Black { 6 } else { 0 }
                                + match pk {
                                    PieceKind::King => 5,
                                    PieceKind::Queen => 4,
                                    PieceKind::Rook => 3,
                                    PieceKind::Knight => 1,
                                    PieceKind::Bishop => 2,
                                    PieceKind::Pawn => 0,
                                },
                        )
                        .unwrap();
                    let res = draw(
                        c,
                        &piece_mesh,
                        DrawParam::default()
                            .dest(ggez::mint::Point2 {
                                x: tile.file as f32 * BOARD_SIZE / BOARD_DIM as f32 + BORDER,
                                y: tile.rank as f32 * BOARD_SIZE / BOARD_DIM as f32 + BORDER,
                            })
                            .scale(ggez::mint::Vector2 { x: 0.6, y: 0.6 }),
                    );
                    if let Err(res) = res{
                        println!("{}",res);
                    }
                }
            }
        }
        let res = present(c);
        if let Err(res) = res{
            println!("{}",res);
        }
    }
    fn _atempt_move_str(&mut self, from: Tile, to: Tile){
        println!("Attempting move: {}", interface::move_to_string(from, to));
        let ap = self
            .game
            .move_from_str(interface::move_to_string(from, to).as_str());
        
        if let Ok(ap) = ap {
            println!("ap ok");
            let res = self.game.perform_action(ap);
            if let Err(res) = res{
                println!("{:?}",res);
            }
            self.display_data = self.game.board.make_display_data();
            self.update_visuals = true;
        
        }
    }
    fn atempt_move(&mut self, from: Tile, to: Tile, promote_to: Option<PieceKind>){
        println!("Attempting move: {}", interface::move_to_string(from, to));
        let ap = self
            .game
            .move_from_gui(from.to_pos(), to.to_pos(), promote_to);
        
        if let Ok(ap) = ap {
            let res = self.game.perform_action(ap);
            if let Err(res) = res{
                if res == "pawn promotion need to be specified"{
                    self.atempt_move(from, to, Some(PieceKind::Queen));
                }

            }
            self.display_data = self.game.board.make_display_data();
            self.update_visuals = true;
        
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, c: &mut Context) -> GameResult {
        if self.texturepack.placeholder {
            self.texturepack = texturepack::make_texturepack(c);
        }

        if self.update_visuals {
            self.render(c);
            self.update_visuals = false;
        }

        Ok(())
    }
    fn mouse_button_down_event(&mut self, _c: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.update_visuals = true;
        println!("{} {}", x, y);
        let clicked_tile = Tile::from_mouse_pos(x, y);
        if clicked_tile.is_some() {
            if self.selected_tile.is_none() {
                self.selected_tile = clicked_tile;
            } else if clicked_tile == self.selected_tile {
                self.selected_tile = None;
            } else {
                if let (Some(from), Some(to)) = (self.selected_tile, clicked_tile) {
                    self.atempt_move(from, to, None);
                }

                self.selected_tile = None;
            }
        }
    }
}

pub fn play_chess(){
    let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("src");
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };
    println!("{:?}", path);

    let window = ggez::ContextBuilder::new("chess", "hansing")
        .window_setup(WindowSetup::default().title("C H E S S"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(BOARD_SIZE + 2.0 * BORDER, BOARD_SIZE + 2.0 * BORDER),
        )
        .add_resource_path(path);
    if let Ok((c, event_loop)) = &mut window.build() {
        let mut state = &mut MainState::new();
        state.display_data = state.game.board.make_display_data();
        let res = event::run(c, event_loop, state);
        if let Err(res) = res{
            println!("{}",res);
        }
    }
}
