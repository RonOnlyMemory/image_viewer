


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Compression {
	Bc1,
	Bc2,
	Bc3,
}

impl Compression {
	pub fn decompress(self, [width, height]: [u32; 2], data: &[u8]) -> Option<Vec<u8>> {
		let width = width as usize;
		let height = height as usize;
		let mut output = vec![0; width as usize*height as usize*4];
		match self {
			Self::Bc1 => {
				squish::Format::Bc1.decompress(data, width, height, &mut output);
			}
			Self::Bc2 => {
				squish::Format::Bc2.decompress(data, width, height, &mut output);
			}
			Self::Bc3 => {
				squish::Format::Bc3.decompress(data, width, height, &mut output);
			}
		}
		Some(output)
	}
}
