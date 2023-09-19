use egui::{Widget, Vec2, Sense, Rect, Pos2, Color32, Stroke};

pub struct ImageViewer<'a> {
	pub pos: Vec2,
	pub scale: Vec2,
	pub texture: &'a egui::TextureHandle,
	pub background_shadow: bool,
}

impl<'a> Widget for ImageViewer<'a> {
	fn ui(self, ui: &mut egui::Ui) -> egui::Response {
		let p2 = ui.ctx().used_size();
		let p = Pos2::new(0.0, 0.0) +self.pos +p2/2.0;
		let s = self.texture.size_vec2()*self.scale;
		let rect = Rect::from_center_size(p, s);
		let painter = ui.painter();
		let fill_color = Color32::from_rgba_unmultiplied(0, 0, 0, 32);
		painter.rect(rect.expand(2.0*self.scale.y).translate(self.scale*0.5), 2.0*self.scale.y, fill_color, Stroke::NONE);
		painter.image(self.texture.id(), rect, Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)), Color32::WHITE);
		let sense = Sense::click_and_drag();
		let response = ui.allocate_rect(rect, sense);
		response
	}
}