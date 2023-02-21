use std::marker::PhantomData;

use super::Module;
use crate::powerline::Segment;

pub struct NewLine<S: NewLineScheme> {
	scheme: PhantomData<S>,
}

pub trait NewLineScheme {}

impl<S: NewLineScheme> NewLine<S> {
	pub fn new() -> NewLine<S> {
		NewLine { scheme: PhantomData }
	}
}

impl<S: NewLineScheme> Module for NewLine<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		segments.push(Segment::new_line());
	}
}
