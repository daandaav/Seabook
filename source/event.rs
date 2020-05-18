pub struct Stream {
	data_stream : RefCell<Data>,
}

pub struct Data {
	data : Vec<usize>,
}

pub enum Event<T> {
	Poll(T),
	Pend,
	
	Actions {
		Wait<T : Sync>,
		Push<T : Send>,
	},
}

trait Stream_t {
	type Input;
	type Output;

	fn pend_for_data(&self : Rc<Stream>, ep : Event::Poll());//amx := Arc<Mutex>

	fn stream_out_data(&self : Rc<Stream>);
}

impl<'a> Stream_t {
	type Input = Event<&'a>;
	type Output = ();

	fn request_input(&self) -> Option<Self::Input>;

	fn pend_for_data(&self : Rc<Stream>, ep : Event::Poll(&'a)) -> Option<Self::Output> {
		//TODO(Stream):= ...stream data by using the std::thread library.
		//\n Rc<>
		Output = (&'a);
		Event::Poll(self.Stream as Output);

		Event::Actions::Wait<&'a>;//Sync<>

		Event::Pend
	}

	fn stream_out_data(&self : Rc<Stream>) -> Result<()> {

		Event::Actions::Push<&'a>//Send<>
	}
	
}

/*
	Event takes input from Data which is derived from Collection
*/