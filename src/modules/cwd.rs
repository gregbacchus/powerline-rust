use std::{env, marker::PhantomData, path};

use dirs::home_dir;

use crate::{terminal::Color, Segment, TextSegment};

use super::Module;

pub struct Cwd<S: CwdScheme> {
	max_length: usize,
	wanted_seg_num: usize,
	resolve_symlinks: bool,
	scheme: PhantomData<S>,
}

pub trait CwdScheme {
	const CWD_FG: Color;
	const PATH_FG: Color;
	const PATH_BG: Color;
	const HOME_FG: Color;
	const HOME_BG: Color;
	const SEPARATOR_FG: Color;
	const CWD_HOME_SYMBOL: &'static str = "";
}

impl<S: CwdScheme> Cwd<S> {
	pub fn new(max_length: usize, wanted_seg_num: usize, resolve_symlinks: bool) -> Cwd<S> {
		Cwd { max_length, wanted_seg_num, resolve_symlinks, scheme: PhantomData }
	}
}

macro_rules! append_cwd_segments {
	($segments: ident, $iter: expr) => {
		for val in $iter {
			$segments.push(Segment::Text(TextSegment::special(
				format!(" {} ", val),
				S::PATH_FG,
				S::PATH_BG,
				'\u{E0B1}',
				S::SEPARATOR_FG,
			)));
		}
	};
}

impl<S: CwdScheme> Module for Cwd<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		let current_dir = if self.resolve_symlinks {
			env::current_dir().unwrap_or_default()
		} else {
			path::PathBuf::from(env::var("PWD").unwrap_or_else(|_| String::from("")))
		};

		let mut cwd = current_dir.to_str().unwrap();

		if let Some(home_path) = home_dir() {
			let home_str = home_path.to_str().unwrap();

			if cwd.starts_with(home_str) {
				segments.push(Segment::Text(TextSegment::simple(
					format!(" {}  ", S::CWD_HOME_SYMBOL),
					S::HOME_FG,
					S::HOME_BG,
				)));
				cwd = &cwd[home_str.len()..]
			}
		}

		let depth = cwd.matches('/').count();
		if (cwd.len() > self.max_length as usize) && (depth > self.wanted_seg_num) {
			let left = self.wanted_seg_num / 2;
			let right = self.wanted_seg_num - left;

			let start = cwd.split('/').skip(1).take(left);
			let end = cwd.split('/').skip(depth - right + 1);

			append_cwd_segments!(segments, start);
			// " ... >"
			segments.push(Segment::Text(TextSegment::special(
				" \u{2026} ",
				S::PATH_FG,
				S::PATH_BG,
				'\u{E0B1}',
				S::SEPARATOR_FG,
			)));
			append_cwd_segments!(segments, end);
		} else {
			append_cwd_segments!(segments, cwd.split('/').skip(1));
		};

		// todo get rid of me
		if let Some(last) = segments.last_mut() {
			match last {
				Segment::NewLine => {},
				Segment::Text(last_text) => {
					if &last_text.val == "  " {
						last_text.val = " / ".to_string()
					}

					last_text.fg = S::CWD_FG.into_fg();
					last_text.sep = '\u{E0B0}';
					last_text.sep_col = last_text.bg.transpose();
				},
			}
		}
	}
}
