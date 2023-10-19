use std::{path::PathBuf, ffi::OsStr};

use image::ImageFormat;

pub fn file(path: &PathBuf) -> bool {
	if extension(path.extension()) {
		return true;
	}
	if let Some(_) = wic_file(path.as_path().to_str()) {
		return true;
	}
	false
}

#[cfg(windows)]
fn wic_file(path: Option<&str>) -> Option<()> {
	let time = std::time::Instant::now();
	dbg!(time.elapsed());
	let decoder = native_windows_gui::ImageDecoder::new().ok()?;
	dbg!(time.elapsed());
	decoder.from_filename(path?).ok()?;
	dbg!(time.elapsed());
	Some(())
}

#[cfg(not(windows))]
fn wic_file(path: Option<&str>) -> Option<()> {
	None
}

fn extension(p: Option<&OsStr>) -> bool {
	if p.is_none() {
		return false;
	}
	let p = p.unwrap();
	let p = p.to_str().unwrap().to_ascii_lowercase();
	match &p[..] {
		"svg" => true,
		"jxl" => true,
		"jfif" => true,
		"vtf" => true,
		"dds" => true,
		"arw" => true,
		_ => ImageFormat::from_extension(p).map(|_| true).unwrap_or(false),
	}
}
