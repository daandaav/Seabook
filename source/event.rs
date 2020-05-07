use std::prelude::v1::*;

#[derive(Debug, Copy)]
enum Event<T, S> {
	message(T, &'static str, bool),
	state(S)
}

trait observeEventMessage_t<T, S> {
	fn observe_event_message(self : &Self, stat : &'static str, active : bool) -> Result<()>;
}

impl<T> observeEventMessage_t<T, S> for Event<T, S> {
	fn observe_event_message(self : &Self, stat : &'static str, active : bool) -> Result<()> {
		match Self {
			Event::message(ref self)=> Self.iter()
		}
	}//fn define_event_message
}

trait callOfEvent_t<T> {
	fn callout_event(Self, active : bool) -> Result;
	fn callback_event(Self, active : bool) -> Result<()>;
}

impl<T> callOfEvent_t for Event {
	fn callout_event(Self, active : bool) -> Result<()> {
		Event::callout_event(Self, active = true)
	}

	fn callback_event(Self, active : bool) -> Result<()> {
		Event::callback_event(Self, active = false)
	}
}

#[derive(Default, Copy)]
enum Data<T> {
	data(T),
}

enum Push<T> {
	push(T),
}

impl<T : Read> Push for Data -> Result<usize> {
	fn data_push_into_iter(Self) -> Result<usize> {
		match Self {
			Data::data(ref Self) => Self.read()
	}
}

enum Pull<T> {
	pull(T),
}

impl<T : Drop> Pull for Data -> Result<usize> {
	fn data_pull_from_iter(Self) -> Result<usize> {
		match Self {
			Data::data(ref Self) => Self.drop()
		}
	}
}