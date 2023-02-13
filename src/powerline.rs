use std::fmt;

use crate::{modules::Module, terminal::*};

const NO_COLOR: Color = Color(0);

#[derive(Clone)]
pub struct Segment {
	pub val: String,
	pub fg: FgColor,
	pub bg: BgColor,
	pub sep: char,
	pub sep_col: FgColor,
	pub no_bg: bool,
}

impl Segment {
	pub fn simple<S: Into<String>>(val: S, fg: Color, bg: Color) -> Segment {
		Segment {
			val: val.into(),
			fg: fg.into_fg(),
			bg: bg.into_bg(),
			sep: '\u{E0B0}',
			sep_col: bg.into_fg(),
			no_bg: false,
		}
	}

	pub fn special<S: Into<String>>(val: S, fg: Color, bg: Color, sep: char, sep_col: Color) -> Segment {
		Segment {
			val: val.into(),
			fg: fg.into_fg(),
			bg: bg.into_bg(),
			sep,
			sep_col: sep_col.into_fg(),
			no_bg: false,
		}
	}

	pub fn no_bg<S: Into<String>>(val: S, fg: Color, sep: char, sep_col: Color) -> Segment {
		Segment {
			val: val.into(),
			fg: fg.into_fg(),
			bg: NO_COLOR.into_bg(),
			sep,
			sep_col: sep_col.into_fg(),
			no_bg: true,
		}
	}

	pub fn char_no_bg(sep: char, sep_col: Color) -> Segment {
		Segment {
			val: "".into(),
			fg: NO_COLOR.into_fg(),
			bg: NO_COLOR.into_bg(),
			sep,
			sep_col: sep_col.into_fg(),
			no_bg: true,
		}
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
			if let Some(next) = iter.peek() {
				if seg.no_bg {
					write!(f, "{}{}{}{}{}{}", seg.fg, Reset, seg.val, Reset, seg.sep_col, seg.sep)?;
				// write!(f, "{}", seg.sep)?;
				} else if next.no_bg {
					write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, Reset, seg.sep_col, seg.sep)?;
				} else {
					write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, next.bg, seg.sep_col, seg.sep)?;
				}
			} else {
				write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, Reset, seg.sep_col, seg.sep)?;
			}
		}
		write!(f, "{} ", Reset)
	}
}
