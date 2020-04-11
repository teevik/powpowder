use crate::{Color};
use crate::live_tile_api::LiveTileApi;
use crate::tile::Tile;
use crate::live_tile_instrution::LiveTileInstruction;

#[derive(Clone, Copy, Debug)]
pub struct TilePosition {
    pub x: u32,
    pub y: u32
}

impl TilePosition {
    pub fn new(x: u32, y: u32) -> TilePosition {
        TilePosition {
            x,
            y
        }
    }
}

pub struct World {
    tiles: Vec<Tile>,
    pub frame: Vec<u8>,
    pub world_width: u32,
    pub world_height: u32
}

impl World {
    pub fn new(initial_color: Color, world_width: u32, world_height: u32) -> World {
        let total_amount_of_tiles: usize = (world_width * world_height) as usize;

        let mut frame: Vec<u8> = Vec::with_capacity(total_amount_of_tiles * 4);

        frame.resize(total_amount_of_tiles * 4, 0);

        for chunk in frame.chunks_exact_mut(4) {
            chunk[0] = initial_color[0];
            chunk[1] = initial_color[1];
            chunk[2] = initial_color[2];
            chunk[3] = 255;
        }

        World {
            tiles: vec![Tile::Empty; total_amount_of_tiles],
            frame,
            world_width,
            world_height
        }
    }

    fn get_tile_index(&self, tile_position: TilePosition) -> usize {
        (tile_position.x + tile_position.y * self.world_width) as usize
    }

    pub fn set_tile(&mut self, tile_position: TilePosition, tile: Tile) {
        let tile_index = self.get_tile_index(tile_position);
        self.tiles[tile_index] = tile;
        let color = tile.get_color();

        self.frame[(tile_index * 4) + 0] = color[0];
        self.frame[(tile_index * 4) + 1] = color[1];
        self.frame[(tile_index * 4) + 2] = color[2];
        self.frame[(tile_index * 4) + 3] = 255;
    }

    pub fn get_tile(&self, tile_position: TilePosition) -> Tile {
        let tile_index = self.get_tile_index(tile_position);
        self.tiles[tile_index]
    }

    pub fn update(&mut self, current_frame: u64) {
        for x in 0..self.world_width {
            for y in 0..self.world_height {
                let tile_position = TilePosition::new(x, y);
                let tile_index = self.get_tile_index(tile_position);

                let tile = &mut self.tiles[tile_index];

                match tile {
                    Tile::Empty => {},
                    Tile::StaticTile(_) => {},
                    Tile::LiveTile(mut live_tile) => {
                        if live_tile.last_frame_updated == current_frame {
                            continue;
                        }

                        let a = live_tile.data.update(LiveTileApi {
                            x,
                            y,
                            world: self
                        });
                        
                        match a {
                            LiveTileInstruction::None => {
                                self.tiles[tile_index] = Tile::LiveTile(live_tile);
                            },
                            LiveTileInstruction::Replace(x_offset, y_offset) => {
                                let nx = ((x as i32) + x_offset) as u32;
                                let ny = ((y as i32) + y_offset) as u32;

                                live_tile.last_frame_updated = current_frame;

                                self.set_tile(TilePosition::new(x, y), Tile::Empty);
                                self.set_tile(TilePosition::new(nx, ny), Tile::LiveTile(live_tile))
                            },
                            LiveTileInstruction::Switch(x_offset, y_offset) =>  {
                                let nx = ((x as i32) + x_offset) as u32;
                                let ny = ((y as i32) + y_offset) as u32;

                                live_tile.last_frame_updated = current_frame;
                                
                                let other_tile_index = self.get_tile_index(TilePosition::new(nx, ny));
                                let other_tile = self.tiles[other_tile_index];

                                self.set_tile(TilePosition::new(x, y), other_tile);
                                self.set_tile(TilePosition::new(nx, ny), Tile::LiveTile(live_tile))
                            }
                            LiveTileInstruction::ReplaceSelfWith(replacement_tile) => {
                                self.set_tile(TilePosition::new(x, y), replacement_tile);
                            }
                        }
                    }
                }
            }
        }
    }
}
