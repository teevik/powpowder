use palette::rgb::Rgb;
use crate::{Color};
use palette::{Lch, Gradient};
use rand::{thread_rng, Rng, random};
use lazy_static::lazy_static;
use crate::tile::{Tile, LiveTile, LiveTileData};
use cgmath::Vector2;
use crate::world::{LiveTileInstruction, LiveTileApi};

lazy_static! {
    static ref SAND_GRADIENT: Gradient<Lch> = Gradient::new(vec![
        Lch::new(78.0, 25.0, 92.0),
        Lch::new(83.0, 25.0, 92.0)
    ]);
    
    static ref WATER_GRADIENT: Gradient<Lch> = Gradient::new(vec![
        Lch::new(65.0, 37.0, 249.0),
        Lch::new(70.0, 37.0, 249.0)
    ]);
}

fn color_from_rgb(color: Rgb) -> Color {
    let (r, g, b) = color.into();
    let r: u8 = (r * 255.0) as u8;
    let g: u8 = (g * 255.0) as u8;
    let b: u8 = (b * 255.0) as u8;

    [r, g, b]
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SandTile {
    pub color: Color,
    pub under_water_ticks: u16
}

impl SandTile {
    pub fn new() -> Self {
        let color: Lch = SAND_GRADIENT.get(thread_rng().gen_range(0.0, 1.0));
        let color: Rgb = color.into();
        
        SandTile {
            color: color_from_rgb(color),
            under_water_ticks: 0
        }
    }

    pub fn update(&mut self, api: LiveTileApi) -> LiveTileInstruction {
        if api.get(Vector2::new(0, 1)) == Tile::Empty {
            return LiveTileInstruction::Replace(Vector2::new(0, 1));
        } else {
            let random_direction = if random() { -1 } else { 1 };
            if api.get(Vector2::new(random_direction, 1)) == Tile::Empty {
                return LiveTileInstruction::Replace(Vector2::new(random_direction, 1));
            }
        }
        
        if let Tile::LiveTile(LiveTile { data: LiveTileData::Water(_), last_frame_updated: _ }) = api.get(Vector2::new(0, 1)) {
            if self.under_water_ticks > 3 {
                self.under_water_ticks = 0;
                return LiveTileInstruction::Switch(Vector2::new(0, 1));
            } else {
                self.under_water_ticks += 1;
            }
        }

        LiveTileInstruction::None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WaterTile {
    pub color: Color
}

impl WaterTile {
    pub fn new() -> WaterTile {
        let color: Lch = WATER_GRADIENT.get(thread_rng().gen_range(0.0, 1.0));
        let color: Rgb = color.into();

        WaterTile {
            color: color_from_rgb(color)
        }
    }

    pub fn update(&mut self, api: LiveTileApi) -> LiveTileInstruction {
        let random_direction = thread_rng().gen_range(-2, 2);
        
        if api.is_empty(Vector2::new(0, 1)) { return LiveTileInstruction::Replace(Vector2::new(0, 1)); } 
        else if api.is_empty(Vector2::new(random_direction, 1)) { return LiveTileInstruction::Replace(Vector2::new(random_direction, 1)); } 
        else if api.is_empty(Vector2::new(-random_direction, 1)) { return LiveTileInstruction::Replace(Vector2::new(-random_direction, 1)); } 
        else if api.is_empty(Vector2::new(random_direction, 0)) { return LiveTileInstruction::Replace(Vector2::new(random_direction, 0)); } 
        else if api.is_empty(Vector2::new(-random_direction, 0)) { return LiveTileInstruction::Replace(Vector2::new(-random_direction, 0)); }

        LiveTileInstruction::None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ParticleTile {
    pub color: Color,
    pub tile: &'static Tile,
    pub velocity: (f32, f32),
    pub offset_wanted: (f32, f32)
}

impl ParticleTile {
    pub fn new(tile: &'static Tile, velocity: (f32, f32)) -> Self {
        let color = tile.get_color();

        Self {
            color,
            tile,
            velocity,
            offset_wanted: (0.0, 0.0)
        }
    }

    pub fn update(&mut self, api: LiveTileApi) -> LiveTileInstruction {
        self.velocity.1 += 0.01;
        self.velocity.0 *= 1.0 - 0.01;
        self.velocity.1 *= 1.0 - 0.01;

        self.offset_wanted.0 += self.velocity.0;
        self.offset_wanted.1 += self.velocity.1;
        
        if self.velocity.1 >= 0.0 && !api.is_empty(Vector2::new(0, 1)) {
            if let Tile::LiveTile(LiveTile { data: LiveTileData::Particle(_), last_frame_updated: _ }) = api.get(Vector2::new(0, 1)) {
            } else {
                return LiveTileInstruction::ReplaceSelfWith(*self.tile);
            }
        }
        
        let a = self.offset_wanted.0.floor() as i32;
        let b = self.offset_wanted.1.floor() as i32;
        
        if a >= 1 || b >= 1 {
            self.offset_wanted.0 -= a as f32;
            self.offset_wanted.1 -= b as f32;
            
            if api.is_empty(Vector2::new(a, b)) {
                return LiveTileInstruction::Replace(Vector2::new(a, b));
            }
        }

        LiveTileInstruction::None
    }
}
