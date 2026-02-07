use egui::Color32;

use crate::OxyTiles;

pub fn show(context: &egui::Context, app: &mut OxyTiles) {
    egui::CentralPanel::default().show(context, |ui| {
        let tile_size = app.canvas.tile_size;

        let (_response, painter) = ui.allocate_painter(
            app.canvas.get_world_size(),
            egui::Sense::click()
        );

        // Calculate centered offset to prevent cropping
        let canvas_size = app.canvas.get_world_size();
        let available_size = painter.clip_rect().size();
        let offset = painter.clip_rect().min.to_vec2() + (available_size - canvas_size) * 0.5;

        for y in 0..app.canvas.tile_map.size.y as usize {
            for x in 0..app.canvas.tile_map.size.x as usize {
                let x_pos = offset.x + (x as f32 * tile_size.x);
                let y_pos = offset.y + (y as f32 * tile_size.y);

                let rect = egui::Rect::from_min_size(
                    egui::pos2(x_pos, y_pos),
                    tile_size,
                );

                if ((x + y) % 2) == 0 {
                    painter.rect_filled(rect, egui::CornerRadius::ZERO, egui::Color32::from_rgb(169,169,169));
                } else{
                    painter.rect_filled(rect, egui::CornerRadius::ZERO, egui::Color32::from_rgb(84,84,84));
                }

                if let Some(uv) = app.canvas.tile_map.tiles.get(&(x, y)) {
                    if let Some(texture) = &app.canvas.tile_set.texture {
                        painter.image(texture.id(), rect, *uv, Color32::WHITE);
                    };
                };

                let response = ui.interact(rect, ui.id().with((x, y)), egui::Sense::click());
                if response.clicked() {
                    if let Some(selected_uv) = app.canvas.tile_map.selected_rect {
                        if let Some(texture) = &app.canvas.tile_set.texture {
                            app.canvas.tile_set.texture = Some(texture.clone());
                            app.canvas.tile_map.tiles.insert((x, y), selected_uv);
                        }
                    }
                }


            };
        };
    });
}