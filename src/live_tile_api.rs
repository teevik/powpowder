use crate::world::{World, TilePosition};
use crate::tile::{Tile, StaticTile};

#[derive(Copy, Clone)]
pub struct LiveTileApi<'a> {
    pub x: u32,
    pub y: u32,
    pub world: &'a World,
}

impl<'a> LiveTileApi<'a> {
    pub fn get(self, dx: i32, dy: i32) -> Tile {
        // if dx > 5 || dx < -5 || dy > 5 || dy < -5 {
        //     panic!("oob get");
        // }

        let nx = ((self.x as i32) + dx) as u32;
        let ny = ((self.y as i32) + dy) as u32;

        if nx < 0 || nx > self.world.world_width - 1 || ny < 0 || ny > self.world.world_height - 1 {
            return Tile::StaticTile(StaticTile::new([255, 0, 0]));
        }

        self.world.get_tile(TilePosition::new(nx, ny))
    }
    
    pub fn is_empty(self, dx: i32, dy: i32) -> bool {
        return self.get(dx, dy) == Tile::Empty   
    }
}