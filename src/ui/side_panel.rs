use crate::OxyTiles;
use eframe::egui;
use egui::{
    Color32, CornerRadius, PointerButton, Rect, Stroke, Vec2
};

pub fn show(context: &egui::Context, app: &mut OxyTiles) {
    egui::SidePanel::right("side_panel").show(context, |ui| {
        if let Some(texture) = &app.canvas.tile_set.texture {
            let response = ui.add(egui::Image::new(texture).sense(egui::Sense::click()));
            if let Some(hover_pos) = response.hover_pos() {
                let local_pos = hover_pos - response.rect.min;
                let tile_size = 16.;

                let grid = Vec2::new(
                    (local_pos.x / tile_size).floor(),
                    (local_pos.y / tile_size).floor(),
                );

                let snap_pos = response.rect.min + Vec2::new(
                    grid.x * tile_size, 
                    grid.y * tile_size, 
                );

                let rect = Rect::from_min_size(
                    snap_pos, 
                    Vec2::new(tile_size, tile_size)
                );

                ui.painter().rect_stroke(
                    rect, 
                    CornerRadius::ZERO,
                    Stroke::new(1., Color32::RED),
                    egui::StrokeKind::Outside
                );

                if response.clicked_by(PointerButton::Primary) {
                    app.canvas.selected_rect = snap_pos;
                    let texture_size = texture.size_vec2();
    
                    let uv_min = egui::pos2(
                        (grid.x * tile_size) / texture_size.x,
                        (grid.y * tile_size) / texture_size.y,
                    );
                    let uv_max = egui::pos2(
                        ((grid.x + 1.0) * tile_size) / texture_size.x,
                        ((grid.y + 1.0) * tile_size) / texture_size.y,
                    );

                    app.canvas.tile_map.selected_rect = Some(Rect::from_min_max(uv_min, uv_max));
                };
            };
        } else {
            ui.label("No texture have been loaded");
            if ui.button("Click here to load").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    let image = image::open(path).expect("Invalid path");
    
                    app.canvas.tile_set.texture = Some(context.load_texture("sidebar-texture", 
                        egui::ColorImage::from_rgba_unmultiplied(
                            [image.width() as _, image.height() as _],
                            image.to_rgba8().as_flat_samples().as_slice(),
                        ),
                        egui::TextureOptions::NEAREST
                    ));
                };
            };
        };
    });
}