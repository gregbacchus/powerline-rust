use std::marker::PhantomData;

use super::Module;
use crate::{powerline::Segment, powerline::TextSegment, terminal::Color};

pub struct Cmd<S: CmdScheme> {
	scheme: PhantomData<S>,
}

pub trait CmdScheme {
	const CMD_FG: Color;
	const CMD_BG: Color;
	const CMD_ROOT_SYMBOL: &'static str = "#";
	const CMD_USER_SYMBOL: &'static str = "$";
}

impl<S: CmdScheme> Cmd<S> {
	pub fn new() -> Cmd<S> {
		Cmd { scheme: PhantomData }
	}
}

impl<S: CmdScheme> Module for Cmd<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		let is_root = users::get_current_uid() == 0;
		let special = if is_root { S::CMD_ROOT_SYMBOL } else { S::CMD_USER_SYMBOL };
		segments.push(Segment::Text(TextSegment::simple(format!(" {} ", special), S::CMD_FG, S::CMD_BG)));
	}
}
