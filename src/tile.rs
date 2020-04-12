use crate::{Color, BACKGROUND_COLOR};
use crate::live_tiles::{SandTile, WaterTile, ParticleTile};
use crate::world::{LiveTileInstruction, LiveTileApi};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LiveTile {
    pub data: LiveTileData,
    pub last_frame_updated: u64
}

impl LiveTile {
    pub fn new(data: LiveTileData) -> LiveTile {
        LiveTile {
            data,
            last_frame_updated: 0
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LiveTileData {
    Sand(SandTile),
    Water(WaterTile)
}

impl LiveTileData {
    pub fn get_color(self) -> Color {
        match self {
            LiveTileData::Sand(sand_tile) => sand_tile.color,
            LiveTileData::Water(water_tile) => water_tile.color
        }
    }

    pub fn update(&mut self, api: LiveTileApi) -> LiveTileInstruction {
        match self {
            LiveTileData::Sand(sand_tile) => sand_tile.update(api),
            LiveTileData::Water(water_tile) => water_tile.update(api)
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
            Tile::LiveTile(live_tile) => live_tile.data.get_color()
        }
    }
}
