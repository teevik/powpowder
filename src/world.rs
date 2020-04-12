use crate::{Color};
use crate::tile::{Tile, StaticTile};
use cgmath::{Vector2, ElementWise};

#[derive(Copy, Clone)]
pub struct LiveTileApi<'a> {
    pub tile_position: Vector2<u32>,
    pub world: &'a World,
}

impl<'a> LiveTileApi<'a> {
    pub fn get(self, tile_offset: Vector2<i32>) -> Tile {

        let new_tile_position = tile_offset.add_element_wise(self.tile_position.cast().unwrap());

        if new_tile_position.x < 0 || new_tile_position.x > (self.world.world_width - 1) as i32 || new_tile_position.y < 0 || new_tile_position.y > (self.world.world_height - 1) as i32 {
            return Tile::StaticTile(StaticTile::new([255, 0, 0]));
        }

        self.world.get_tile(new_tile_position.cast().unwrap())
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

pub struct World {
    tiles: Vec<Tile>,
    // particles: Vec
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

    fn get_tile_index(&self, tile_position: Vector2<u32>) -> usize {
        (tile_position.x + tile_position.y * self.world_width) as usize
    }

    pub fn set_tile(&mut self, tile_position: Vector2<u32>, tile: Tile) {
        let tile_index = self.get_tile_index(tile_position);
        self.tiles[tile_index] = tile;
        let color = tile.get_color();

        self.frame[(tile_index * 4) + 0] = color[0];
        self.frame[(tile_index * 4) + 1] = color[1];
        self.frame[(tile_index * 4) + 2] = color[2];
        self.frame[(tile_index * 4) + 3] = 255;
    }

    pub fn get_tile(&self, tile_position: Vector2<u32>) -> Tile {
        let tile_index = self.get_tile_index(tile_position);
        self.tiles[tile_index]
    }

pub fn update(&mut self, current_frame: u64) {
    for x in 0..self.world_width {
        for y in 0..self.world_height {
            let tile_position = Vector2::new(x, y);
            let tile_index = self.get_tile_index(tile_position);

            let tile = &mut self.tiles[tile_index];

            match tile {
                Tile::Empty => {},
                Tile::StaticTile(_) => {},
                Tile::LiveTile(mut live_tile) => {
                        if live_tile.last_frame_updated == current_frame {
                            continue;
                        }

                        let live_tile_instruction = live_tile.data.update(LiveTileApi {
                            tile_position,
                            world: self
                        });
                        
                        match live_tile_instruction {
                            LiveTileInstruction::None => {
                                self.tiles[tile_index] = Tile::LiveTile(live_tile);
                            },
                            LiveTileInstruction::Replace(tile_offset) => {
                                let new_tile_position: Vector2<u32> = tile_offset.add_element_wise(tile_position.cast().unwrap()).cast().unwrap();

                                live_tile.last_frame_updated = current_frame;

                                self.set_tile(tile_position, Tile::Empty);
                                self.set_tile(new_tile_position, Tile::LiveTile(live_tile))
                            }
                            LiveTileInstruction::Switch(tile_offset) =>  {
                                let new_tile_position: Vector2<u32> = tile_offset.add_element_wise(tile_position.cast().unwrap()).cast().unwrap();

                                live_tile.last_frame_updated = current_frame;
                                
                                let other_tile_index = self.get_tile_index(new_tile_position);
                                let other_tile = self.tiles[other_tile_index];

                                self.set_tile(tile_position, other_tile);
                                self.set_tile(new_tile_position, Tile::LiveTile(live_tile))
                            }
                            LiveTileInstruction::ReplaceSelfWith(replacement_tile) => {
                                self.set_tile(tile_position, replacement_tile);
                            }
                        }
                    }
                }
            }
        }
    }
}
