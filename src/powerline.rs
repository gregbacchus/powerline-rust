use std::fmt;

use crate::{modules::Module, terminal::*};

#[derive(Clone)]
pub struct TextSegment {
	pub val: String,
	pub fg: FgColor,
	pub bg: BgColor,
	pub sep: char,
	pub sep_col: FgColor,
}

pub enum Segment {
	NewLine,
	Text(TextSegment),
}

impl Segment {
	pub fn new_line() -> Segment {
		Segment::NewLine
	}

	pub fn simple<S: Into<String>>(val: S, fg: Color, bg: Color) -> Segment {
		Segment::Text(TextSegment {
			val: val.into(),
			fg: fg.into_fg(),
			bg: bg.into_bg(),
			sep: '\u{E0B0}',
			sep_col: bg.into_fg(),
		})
	}

	pub fn special<S: Into<String>>(val: S, fg: Color, bg: Color, sep: char, sep_col: Color) -> Segment {
		Segment::Text(TextSegment {
			val: val.into(),
			fg: fg.into_fg(),
			bg: bg.into_bg(),
			sep,
			sep_col: sep_col.into_fg(),
		})
	}
}

pub struct Powerline {
	segments: Vec<Segment>,
}

impl Powerline {
	pub fn new() -> Powerline {
		Powerline { segments: Vec::new() }
	}

	pub fn add_module(&mut self, mut part: impl Module) {
		part.append_segments(&mut self.segments)
	}
}

impl fmt::Display for Powerline {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.segments.iter().peekable();
		while let Some(seg) = iter.next() {
			match seg {
				Segment::NewLine => {
					write!(f, "{}\n", Reset)?;
				},
				Segment::Text(text) => {
					if let Some(next) = iter.peek() {
						match next {
							Segment::NewLine => {
								write!(
									f,
									"{}{}{}{}{}{}",
									text.fg, text.bg, text.val, Reset, text.sep_col, text.sep
								)?;
							},
							Segment::Text(next_text) => {
								if next_text.bg == text.bg && text.sep_col.transpose() == text.bg {
									// skip separator
									write!(f, "{}{}{}", text.fg, text.bg, text.val)?;
								} else {
									write!(
										f,
										"{}{}{}{}{}{}",
										text.fg, text.bg, text.val, next_text.bg, text.sep_col, text.sep
									)?;
								}
							},
						}
					} else {
						// last segment - no more
						write!(f, "{}{}{}{}{}{}", text.fg, text.bg, text.val, Reset, text.sep_col, text.sep)?;
					}
				},
			}
		}
		write!(f, "{} ", Reset)
	}
}
