use arboard::{Clipboard, ClipboardFormat};

const FORMAT_SPECIAL: &str = "dyn.arboard.pecial.format";

fn main() {
	env_logger::init();

	let mut ctx = Clipboard::new().unwrap();

	let formats = [
		ClipboardFormat::Text,
		ClipboardFormat::Html,
		ClipboardFormat::Rtf,
		#[cfg(feature = "image-data")]
		ClipboardFormat::ImageRgba,
		#[cfg(feature = "image-data")]
		ClipboardFormat::ImagePng,
		#[cfg(feature = "image-data")]
		ClipboardFormat::ImageSvg,
		ClipboardFormat::Special(FORMAT_SPECIAL),
	];
	for d in ctx.get_formats(&formats).unwrap() {
		println!("data: {:?}", d);
	}
}
