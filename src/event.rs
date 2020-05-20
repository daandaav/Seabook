struct Stream {
	data_stream : RefCell<Data>,
}

struct Data {
	data : Vec<usize>,
}

enum Event<T> {
	Poll(T),
	Pend,
	
	Actions {
		Wait : dyn Sync,
		Push : dyn Send,
	},
}

trait StreamT {
	fn pend_for_data(&self, rcs : Rc<Stream>, ep : fn());//amx := Arc<Mutex>

	fn stream_out_data(&self, rcs : Rc<Stream>);
}

impl<'a, T> dyn StreamT {
	type Process = Event<&'a T>;
	type Output = ();

	fn request_input(&self) -> Option<<(dyn event::StreamT + 'static) as Trait>::Process> {
		return self
	}

	fn pend_for_data(&self, rcs : Rc<Stream>, ep : fn()) -> Option<<(dyn event::StreamT + 'static) as Trait>::Output> {
		//TODO(Stream):= ...stream data by using the std::thread library.
		//\n Rc<>
		Event::Poll(self.Stream);

		Event::Wait;

		Event::Pend
	}

	fn stream_out_data(&self, rcs : Rc<Stream>) -> Result<(usize, Vec<Stream>)> {
		Event::Push;
	}
	
}

/*
	Event takes input from Data which is derived from Collection
*/