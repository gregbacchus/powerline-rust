use std::env;
use std::marker::PhantomData;

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

		segments.push(Segment::simple(format!("{}", title), S::FISH_FG, S::FISH_BG));
	}
}
