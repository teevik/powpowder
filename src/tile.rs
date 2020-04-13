use crate::{Color, BACKGROUND_COLOR};
use crate::live_tiles::{SandTile, WaterTile};
use cgmath::{Vector2, ElementWise};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiveTile {
    pub state: LiveTileState,
    pub last_frame_updated: u64
}

impl LiveTile {
    pub fn new(state: LiveTileState) -> LiveTile {
        LiveTile {
            state,
            last_frame_updated: 0
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LiveTileState {
    Sand(SandTile),
    Water(WaterTile)
}

impl LiveTileState {
    pub fn get_color(self) -> Color {
        match self {
            LiveTileState::Sand(sand_tile) => sand_tile.color,
            LiveTileState::Water(water_tile) => water_tile.color
        }
    }

    pub fn update(&mut self, api: LiveTileApi) -> LiveTileInstruction {
        match self {
            LiveTileState::Sand(sand_tile) => sand_tile.update(api),
            LiveTileState::Water(water_tile) => water_tile.update(api)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StaticTile {
    color: Color
}

impl StaticTile {
    pub fn new(color: Color) -> StaticTile {
        StaticTile {
            color
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    StaticTile(StaticTile),
    LiveTile(LiveTile)
}

impl Tile {
    pub fn get_color(self) -> Color {
        match self {
            Tile::Empty => BACKGROUND_COLOR,
            Tile::StaticTile(static_tile) => static_tile.color,
            Tile::LiveTile(live_tile) => live_tile.state.get_color()
        }
    }
}

#[derive(Copy, Clone)]
pub struct LiveTileApi<'a> {
    pub tile_position: Vector2<u32>,
    pub tiles: &'a Vec<Tile>,
    pub world_width: u32,
    pub world_height: u32
}

impl<'a> LiveTileApi<'a> {
    pub fn get(self, tile_offset: Vector2<i32>) -> Tile {

        let new_tile_position = tile_offset.add_element_wise(self.tile_position.cast().unwrap());

        if new_tile_position.x < 0 || new_tile_position.x >= self.world_width as i32 || new_tile_position.y < 0 || new_tile_position.y >= self.world_height as i32 {
            return Tile::StaticTile(StaticTile::new([255, 0, 0]));
        }

        let new_tile_position: Vector2<u32> = new_tile_position.cast().unwrap();
        let tile_index = (new_tile_position.x + new_tile_position.y * self.world_width) as usize;

        self.tiles[tile_index]
    }

    pub fn is_empty(self, tile_offset: Vector2<i32>) -> bool {
        return self.get(tile_offset) == Tile::Empty
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LiveTileInstruction {
    Replace(Vector2<i32>),
    Switch(Vector2<i32>),
    ReplaceSelfWith(Tile),
    None
}

