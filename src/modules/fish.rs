use std::marker::PhantomData;
use std::env;

use super::Module;
use crate::{terminal::Color, Segment};

pub struct Fish<S: FishScheme> {
	scheme: PhantomData<S>,
}

pub trait FishScheme {
	const FISH_BG: Color;
	const FISH_FG: Color;
}

impl<S: FishScheme> Fish<S> {
	pub fn new() -> Fish<S> {
		Fish { scheme: PhantomData }
	}
}

impl<S: FishScheme> Module for Fish<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		let title = env::var("TITLE").unwrap_or("".to_string());

		// segments.push(Segment::simple(
		// 	format!("\u{E0B0}\u{E0B2}"),
		// 	S::FISH_BG,
		// 	S::FISH_FG,
		// ));

		// segments.push(Segment::special(
		// 	"",
		// 	S::FISH_FG,
		// 	S::FISH_BG,
		// 	'\u{E0B0}',
		// 	S::FISH_BG,
		// ));

		// segments.push(Segment::special(
		// 	"",
		// 	S::FISH_FG,
		// 	S::FISH_BG,
		// 	'\u{E0B2}',
		// 	S::FISH_BG,
		// ));

		segments.push(Segment::simple(
			format!(" {} ", title),
			S::FISH_FG,
			S::FISH_BG,
		));

		// segments.push(Segment::special(
		// 	" F ",
		// 	S::FISH_FG,
		// 	S::FISH_BG,
		// 	'\u{E0B1}',
		// 	S::FISH_BG,
		// ));
	}
}
