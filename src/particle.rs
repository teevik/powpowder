use crate::tile::{Tile, LiveTileApi};
use cgmath::{Vector2, ElementWise};
use crate::world::World;

#[derive(Copy, Clone, Debug)]
pub enum ParticleInstructions {
    None,
    TurnIntoTile(Vector2<u32>),
    Destroy
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Particle {
    pub tile: Tile,
    pub position: Vector2<f32>,
    velocity: Vector2<f32>
}

impl Particle {
    pub fn new(tile: Tile, position: Vector2<f32>, velocity: Vector2<f32>) -> Self {
        Self {
            tile,
            position,
            velocity
        }
    }
    
    pub fn update(&mut self, tiles: &Vec<Tile>, world_width: u32, world_height: u32) -> ParticleInstructions {
        self.velocity.y += 0.01;
        self.position.add_assign_element_wise(self.velocity);
        
        let rounded_position: Vector2<u32> = self.position.cast().unwrap();
        
        let api = LiveTileApi {
            tile_position: rounded_position,
            tiles,
            world_width,
            world_height
        };
        
        if !api.is_empty(Vector2::new(0, 1)) {
            for y_offset in 0..10 {
                if api.is_empty(Vector2::new(0, 1 - y_offset)) { 
                    let offset: Vector2<u32> = Vector2::new(0, 1 - y_offset).add_element_wise(rounded_position.cast().unwrap()).cast().unwrap();
                    
                    return ParticleInstructions::TurnIntoTile(offset);
                }
            }
            
            return ParticleInstructions::Destroy;
        }
        
        ParticleInstructions::None
    }
}