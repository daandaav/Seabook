use {
	std::net::{
		SocketAddr,
		TcpListener,
		TcpStream,
	},
	std::rc::Rc,
	std::cell::RefCell,
	std::prelude::v1::*,
};

struct Stream {
	data_stream : Rc<Data>,
}

struct Data {
	data : Vec<usize>,
}

enum Event<T> {
	Poll(T),
	//...
}

trait StreamT {
	fn poll_event_stream(&self, rcs : Rc<Stream>, ep : fn());//amx := Arc<Mutex>
}

impl<'a> dyn StreamT {
	fn poll_event_stream(&self, rcs : Rc<Stream>) {
		//TODO(Stream):= ...stream data by using the std::thread library.
		//\n Rc<>
		match self {
			Event::Poll(rcs)
		};
	}
	
}

/*
	Event takes input from Data which is derived from Collection
*/