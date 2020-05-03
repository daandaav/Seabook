use std::prelude::v1::*;

#[derive(Debug, Copy)]
enum Event {
	type : &'static str,
	active : bool,
}

trait defineEventType_t {
	fn define_event_type(Self) -> Event;
}

impl<T> defineEventType_t <T> for Event<T> {
	fn define_event_type(Self) {
		match Self {
			Event::type => Self
		}
	}
}

trait callOfEvent_t<T> {
	fn callout_event(Self, active : bool) -> Result;
	fn callback_event(Self, active : bool) -> Result;
}

impl<T> callOfEvent_t for Event {
	fn callout_event(Self, active : bool) -> Result {
		Event::callout_event(Self, active = true)
	}

	fn callback_event(Self, active : bool) -> Result {
		Event::callback_event(Self, active = false)
	}
}