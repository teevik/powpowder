use crate::{Color};
use crate::tile::{Tile, LiveTileApi, LiveTileInstruction};
use cgmath::{Vector2, ElementWise};
use crate::particle::{Particle, ParticleInstructions};
use retain_mut::RetainMut;

pub struct World {
    tiles: Vec<Tile>,
    particles: Vec<Particle>,
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
            particles: Vec::new(),
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
    
    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
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

                        let live_tile_instruction = live_tile.state.update(LiveTileApi {
                            tile_position,
                            tiles: &self.tiles,
                            world_width: self.world_width,
                            world_height: self.world_height
                        });
                        
                        match live_tile_instruction {
                            LiveTileInstruction::None => {
                                self.set_tile(tile_position, Tile::LiveTile(live_tile))
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
        
        let tiles = &self.tiles;
        let world_width = self.world_width;
        let world_height = self.world_height;
        
        let mut new_tiles: Vec<(Vector2<u32>, Tile)> = Vec::new();

        for i in (0..self.particles.len()).rev() {
            let particle = &mut self.particles[i];
            
            let particle_instructions = particle.update(tiles, world_width, world_height);
            
            match particle_instructions {
                ParticleInstructions::None => {},
                ParticleInstructions::Destroy => {
                    self.particles.remove(i);
                },
                ParticleInstructions::TurnIntoTile(tile_position) => {
                    new_tiles.push((tile_position, particle.tile));
                    self.particles.remove(i);
                }
            }
        }

        for (tile_position, tile) in new_tiles {
            self.set_tile(tile_position, tile);
        }
    }
    
    pub fn render(&mut self, frame: &mut[u8]) {
        frame.copy_from_slice(&self.frame);

        let world_width = self.world_width;
        let world_height = self.world_height;
        
        self.particles.retain_mut(|particle| {
            let position = particle.position;
            let color = particle.tile.get_color();
            
            if position.x < 0.0 || position.y < 0.0 || position.x >= world_width as f32 || position.y >= world_height as f32 {
                return false;
            }
            
            let position: Vector2<u32> = position.cast().unwrap();
            let frame_index = (position.x + position.y * world_width) as usize;

            frame[frame_index * 4 + 0] = color[0];
            frame[frame_index * 4 + 1] = color[1];
            frame[frame_index * 4 + 2] = color[2];
            frame[frame_index * 4 + 3] = 255;
            
            true
        });
    }
}
