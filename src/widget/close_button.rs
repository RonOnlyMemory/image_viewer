use egui::{Widget, Response, FontId, Pos2, Color32, Stroke, Align2, Rect, Vec2, Sense};

pub struct CloseButton;

impl Widget for CloseButton {
	fn ui(self, ui: &mut egui::Ui) -> Response {
		let screen_size = ui.ctx().used_size();

		let rect = Rect::from_min_max(
			Pos2::new(-48.0, 0.0),
			Pos2::new(0.0, 48.0),
		).translate(Vec2::new(screen_size.x, 0.0));
		let sense = Sense::click();
		let response = ui.allocate_rect(rect, sense);
		
		let mut bg_color = Color32::from_rgba_unmultiplied(0, 0, 0, 64);
		let mut fg_color = Color32::from_rgba_unmultiplied(192 +32, 192 +32, 192 +32, 255);
		let fg_2_color = Color32::from_rgba_unmultiplied(64, 64, 64, 255);
		if response.hovered() {
			bg_color = Color32::from_rgba_unmultiplied(16, 16, 16, 128);
			fg_color = Color32::from_rgba_unmultiplied(255, 255, 255, 255);
		}

		let painter = ui.painter();
		let font_id = FontId::new(24.0, egui::epaint::FontFamily::Monospace);
		let font_id_2 = FontId::new(25.0, egui::epaint::FontFamily::Monospace);
		painter.circle(Pos2::new(screen_size.x, 0.0), 48.0, bg_color, Stroke::NONE);
		painter.text(Pos2::new(screen_size.x -16.0, 16.0), Align2::CENTER_CENTER, "x", font_id_2.clone(), fg_2_color);
		painter.text(Pos2::new(screen_size.x -16.0, 16.0), Align2::CENTER_CENTER, "x", font_id, fg_color);

		response
	}
}
