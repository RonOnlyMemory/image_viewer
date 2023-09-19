use crate::animation::{Animation, Frame};

pub struct AnimationPlayer {
	pub animation: Option<Animation>,
	pub time: f64,
	pub index: u32,
}

impl AnimationPlayer {
	pub fn new() -> Self {
		Self {
			animation: None,
			time: 0.0,
			index: 0,
		}
	}
	pub fn clear(&mut self) {
		*self = Self {
			animation: None,
			time: 0.0,
			index: 0,
		};
	}
	pub fn current_frame_clone(&self) -> Option<Frame> {
		let a = self.animation.as_ref()?;
		Some(a.frames[self.index as usize].clone())
	}
	pub fn current_frame(&self) -> Option<&Frame> {
		let a = self.animation.as_ref()?;
		Some(&a.frames[self.index as usize])
	}
	pub fn advance(&mut self, d: f64) {
		self.time += d;
		if let Some(Frame { delta, .. }) = self.current_frame_clone() {
			while self.time > delta {
				self.time -= delta;
				self.index = (self.index +1)%self.animation.as_ref().unwrap().frames.len() as u32;
			}
		}
	}
}
