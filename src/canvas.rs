use egui::{Pos2, Vec2};

use crate::{
    tile_map::TileMap, 
    tile_set::TileSet
};

pub struct Canvas {
    pub tile_size: Vec2,
    pub tile_map: TileMap,
    pub tile_set: TileSet,
    pub selected_rect: Pos2,
}
impl Canvas {
	// Constants
	// Constructors
	// Public functions
    pub fn get_tile_size(&self, zoom: f32) -> Vec2 {
        self.tile_size * zoom
    }
    pub fn get_world_size(&self) -> Vec2 {
        Vec2 { 
            x: self.tile_map.size.x * self.tile_size.x, 
            y: self.tile_map.size.y * self.tile_size.y,
        }
    }
	// Private functions
}
impl Default for Canvas {
    fn default() -> Self {
        Self {
            tile_size: Vec2::splat(32.),
            tile_map: TileMap::new(Vec2::splat(8.)),
            tile_set: TileSet::new(Vec2::splat(4.)),
            selected_rect: Pos2::ZERO,
        }
    }
}
