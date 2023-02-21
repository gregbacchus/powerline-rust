use std::marker::PhantomData;
use std::path::Path;

use crate::terminal::Color;
use crate::{Segment, TextSegment};

use super::Module;

pub struct PyVenv<S: PyVenvScheme> {
	scheme: PhantomData<S>,
}

pub trait PyVenvScheme {
	const PYVENV_FG: Color;
	const PYVENV_BG: Color;
	const PYVENV_SYMBOL: &'static str = "🐍";
}

impl<S: PyVenvScheme> PyVenv<S> {
	pub fn new() -> PyVenv<S> {
		PyVenv { scheme: PhantomData }
	}
}

impl<S: PyVenvScheme> Module for PyVenv<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		let pyvenv = std::env::var("VIRTUAL_ENV")
			.or_else(|_| std::env::var("CONDA_ENV_PATH"))
			.or_else(|_| std::env::var("CONDA_DEFAULT_ENV"));
		pyvenv
			.map(|venv| {
				if let Some(venv_name) = Path::new(&venv).file_name() {
					segments.push(Segment::Text(TextSegment::simple(
						format!(" {} {} ", S::PYVENV_SYMBOL, venv_name.to_string_lossy()),
						S::PYVENV_FG,
						S::PYVENV_BG,
					)));
				}
			})
			.unwrap_or_default();
	}
}
