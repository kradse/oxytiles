use std::collections::HashMap;

use egui::{Rect, Vec2};

// Used in editor
pub struct TileMap {
    pub size: Vec2,
    pub selected_rect: Option<Rect>,
    pub tiles: HashMap<(usize, usize), egui::Rect>,
}
impl TileMap {
	// Constants
	// Constructors
    pub fn new(size: Vec2) -> TileMap
    {
        TileMap { 
            size,
            selected_rect: None,
            tiles: HashMap::new(),
        }
    }
	// Public functions
	// Private functions
}