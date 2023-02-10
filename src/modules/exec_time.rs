use std::env;
use std::marker::PhantomData;

use super::Module;
use crate::{terminal::Color, Segment};

pub struct ExecTime<S: ExecTimeScheme> {
	scheme: PhantomData<S>,
}

pub trait ExecTimeScheme {
	const EXEC_TIME_BG: Color;
	const EXEC_TIME_FG: Color;
}

impl<S: ExecTimeScheme> ExecTime<S> {
	pub fn new() -> ExecTime<S> {
		ExecTime { scheme: PhantomData }
	}
}

fn hms(duration_ms: i32) -> String {
	let ms = duration_ms % 1000;

	let duration_s = duration_ms / 1000;
	let s = duration_s % 60;

	let duration_m = duration_s / 60;
	let m = duration_m % 60;

	let duration_h = duration_m / 60;

	if duration_h > 0 {
		format!("{}h {}m {}s {}ms", duration_h, m, s, ms)
	} else if m > 0 {
		format!("{}m {}s {}ms", m, s, ms)
	} else if s > 0 {
		format!("{}s {}ms", s, ms)
	} else {
		format!("{}ms", ms)
	}
}

impl<S: ExecTimeScheme> Module for ExecTime<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		let duration_string = env::var("CMD_PREV_EXEC_MS").unwrap_or("".to_string());
		let duration = duration_string.parse::<i32>().unwrap_or_default();

		if duration == 0 {
			return;
		}

		segments.push(Segment::simple(format!(" {} ", hms(duration)), S::EXEC_TIME_FG, S::EXEC_TIME_BG));
	}
}
