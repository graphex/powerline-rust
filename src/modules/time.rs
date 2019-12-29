use std::marker::PhantomData;

use super::Module;
use crate::{powerline::Segment, terminal::Color, R};
use chrono::Local;

pub struct Time<S>(PhantomData<S>);

pub trait TimeScheme {
	const TIME_FG: Color;
	const TIME_BG: Color;
	const TIME_FORMAT_STRING: &'static str = "%T";
}

impl<S: TimeScheme> Time<S> {
	pub fn new() -> Time<S> {Time(PhantomData)}
}

impl<S: TimeScheme> Module for Time<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let fg = S::TIME_FG;
		let bg = S::TIME_BG;
		let now = Local::now().format(S::TIME_FORMAT_STRING).to_string();
		segments.push(Segment::simple(format!(" {} ", now), fg, bg));
		Ok(())
	}
}
