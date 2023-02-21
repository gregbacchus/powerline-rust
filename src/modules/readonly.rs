use std::{ffi::CString, marker::PhantomData};

use crate::{terminal::Color, Segment};

use super::Module;

pub struct ReadOnly<S>(PhantomData<S>);

pub trait ReadOnlyScheme {
	const READONLY_FG: Color;
	const READONLY_BG: Color;
	const READONLY_SYMBOL: &'static str = "";
}
impl<S: ReadOnlyScheme> ReadOnly<S> {
	pub fn new() -> ReadOnly<S> {
		ReadOnly(PhantomData)
	}
}

impl<S: ReadOnlyScheme> Module for ReadOnly<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		let readonly = CString::new("./")
			.map(|path| unsafe { libc::access(path.as_ptr(), libc::W_OK) != 0 })
			.unwrap_or(false);

		if readonly {
			segments.push(Segment::simple(format!("{}", S::READONLY_SYMBOL), S::READONLY_FG, S::READONLY_BG));
		}
	}
}
