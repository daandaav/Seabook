use {
	std::net::{
		SocketAddr,
		TcpListener,
		TcpStream,
	},
	std::prelude::v1::*,
};

enum Collection<T> {
	Data(T),
	Await,

	Pointer {
		//...iterate through a collection
		dataset_alloc : Vec<u32>,
		iter_thru : fn(T),
	},
}

trait CollectionT {
	fn write_copy(&self, rwc : Vec<usize, Collection>) -> Result<()>;

	fn read_copy(&self, rwc : Vec<usize, Collection>);
}

impl<'a> dyn CollectionT {
	//...
	fn write_copy(&self, rwc : Vec<usize, Collection>) -> Result<()> {
		self.rwc
			.write(rwc)
			.clone();//write into copy
	}
	
	fn read_copy(&self, rwc : Vec<usize, Collection>) {
		match self {
			Collection::Data(rwc) => self.read()
		}//match self
		//...
	}//fn read_copy(&self)
}

struct Message {
	data_collection : RefCell<Collection>,

	buffer : Vec<u32>,

	read_only : Rc<Collection>,
}

trait InterMsgQT {
	type Msg = RefCell<Message>;

	fn read_interface_msg(&self, c : Vec<Collection>, m : Message::read_only<Self::Msg>);

	fn man_buffer_size(&self, d : <coll::Message as Trait>::data_collection, b : <coll::Message as Trait>::buffer);
}

impl<'a> dyn InterMsgQT {
	type Msg = RefCell<Message>;

		fn read_interfacing_msg(
			&self,
			c : Vec<usize, Collection>,
			m : <coll::Message as Trait>::read_only`,
		) {
			let c = Vec::with_capacity(0..254);
			
			if let i = 0u32 < self.len() {
			//TODO:...
				self.iter(m)
				.map(|h| h.read_only)
				.next()
			} else { return 1?; }
			//...
		}//fn read_interfacing_msg

		fn man_buffer_size(
			&self,
			d : <coll::Message as Trait>::data_collection,
			b : <coll::Message as Trait>::buffer,
		) {
			let i = 0u32;
			if i < d.iter() {
				let b = Vec::new();
				self.iter(d)
					.map(|h| h.buffer)
					.sort() } else if i > d.iter() {
						b.clear();
						return 0;
					} //TODO: b.clear()
		}//fn man_buffer_size
}

trait HandlerT {
	fn handle_client(s : TcpStream);

	fn client_main_handle() -> std::io::Result<()>;
}

impl<'a> dyn HandlerT {

	fn handle_client(&self, tcps : TcpStream) {
		//TODO(Client Handler): ...
		self.tcps = std::thread::spawn(move || {
			tcps.send()
				.to_owned()
				.expect("[!?] Unable to send [?!]");
		})
	}

	fn client_main_handle() -> std::io::Result<()>{
		let sock_addrs = [
			SocketAddr::from(([127,0,0,1], 80)),
			SocketAddr::from(([127,0,0,1], 443)),
			SocketAddr::from(([127,0,0,1], 3000)),
		];
	
		let listener = TcpListener::bind(&sock_addrs[..]).unwrap()?;
	
		for tcps in listener.incoming() {
			HandlerT::handle_client(tcps: TcpStream)?;
	
			match tcps {
				Ok(tcps) => {
					println!("Client - NEW: {:?}", tcps);
				}
				Err(e) => println!("Couldn't get client (with handler): {:?}", e),
			}//match tcps
		}//for tcps in listener.incoming()
	}//fn client_main_handle()
}//impl<'a> HandlerT