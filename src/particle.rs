#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    StaticTile(StaticTile),
    LiveTile(LiveTile)
}
