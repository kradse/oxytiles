#![allow(dead_code)]
use eframe::egui;

use crate::canvas::Canvas;

mod tile_map;
mod tile_set;
mod camera;
mod canvas;
mod ui;

// use tile_map::TileMap;
// use tile_set::TileSet;
// use camera::Camera;
// use canvas::Canvas;

fn main() -> eframe::Result {
    eframe::run_native("OxyTiles", 
        eframe::NativeOptions { 
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([960., 720.])
                // .with_decorations(false) // implement this later on
                .with_transparent(true),
                ..Default::default()
            
        },
        Box::new(|creation_context| {
            egui_extras::install_image_loaders(&creation_context.egui_ctx);
            Ok(Box::<OxyTiles>::default())
        })
    )
}

struct OxyTiles {
	pub canvas: Canvas,
}
impl OxyTiles {
	// Constants
	// Constructors
	// Public functions
	// Private functions
}
impl Default for OxyTiles {
    fn default() -> Self {
        Self {
            canvas: Canvas::default(),
        }
    }
}
impl eframe::App for OxyTiles {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::show(ctx, self);
    }
}