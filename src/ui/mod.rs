use crate::OxyTiles;

pub mod editor;
pub mod side_panel;

pub fn show(context: &egui::Context, app: &mut OxyTiles)
{
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(30, 30, 30);
    context.set_visuals(visuals);

    editor::show(context, app);
    side_panel::show(context, app);
}