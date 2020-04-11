use crate::tile::Tile;

#[derive(Copy, Clone, Debug)]
pub enum LiveTileInstruction {
    Replace(i32, i32),
    Switch(i32, i32),
    ReplaceSelfWith(Tile),
    None
}