use egui::Vec2;
// used in sidebar
pub struct TileSet {
    pub size: Vec2,
	pub texture: Option<egui::TextureHandle>,
}
impl TileSet {
	// Constants
	// Constructors
	pub fn new(size: Vec2) -> TileSet {
        TileSet { 
			size,
			texture: None,
		}
    }
	// Public functions
	// Private functions
}