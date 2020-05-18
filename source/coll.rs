use {
	std::net::{
		SocketAddr,
		TcpListener,
		TcpStream,
	},
	std::prelude:v1::*,
};

enum Collection<'a> {
	Data(T),
	Await,

	Pointer {
		//...iterate through a collection
		dataset_alloc : Vec<u32>,
		iter_thru('a),
	},
}

impl<R : Read, W : Write, 'a> where 'a: Collection {
	//...
	fn write_copy(&self : Collection::Data(W), rwc : &'a) -> Collection {
		self = Option<Self::Something = (rwc)>;//write into copy
	}
	
	fn read_copy(&self : Collection::Data(R)) {
		match self {
			Collection::Data(&'a) => self.read(R)
		}//match self
		//...
	}//fn read_copy(&self)
}

struct Message {
	data_collection : Vec<Collection>,

	buffer : Vec<u32>,

	read_only : Rc<Collection>,
}

trait InterMsgQ_t {
	type Msg;

	fn read_interface_msg(&self : Vec<Collection>, m : Message::read_only<Self::Msg>);

	fn man_buffer_size(&self, b : Message::buffer);
}

impl<'a> InterMsgQ_t {
	type Msg = Collection<&'a>;

		fn read_interfacing_msg(
			&self : Vec<Collection>,
			m : Message::read_only,
		) {
			let self : Vec<Collection> = Vec.with_capacity(0..254);
			
			if (let i = 0u32) < self.len() {
			//TODO:...
				self.iter(m)
				.map(|h| h.read_only)
				.next()
			} else { return?; }
			//...
		}//fn read_interfacing_msg

		fn man_buffer_size(
			&self,
			d : &Message::data_collection,
			b : &Message::buffer,
		) {
			let i = 0u32;
			if i < d.iter() {
				let b = Vec::new();
				self.iter(d)
					.map(|h| h.buffer)
					.sort() } else if i > d.iter() {
						b.clear();
						break;
					} //TODO: b.clear()
		}//fn man_buffer_size
}

trait Handler_t {
	fn handle_client(s : TcpStream);

	fn client_main_handle() -> std::io::Result<()>;
}

impl<'a> Handler_t {
	type Returning_Value = ();

	fn handle_client(&self, tcps : TcpStream) {
		//TODO(Client Handler): ...
		let Returning_Value = (&'a);

		self.tcps = std::thread::spawn(move || {
			tcpss.send(Returning_Value.to_owned())
				.expect("[!?] Unable to send [?!]");
		})
	}

	fn client_main_handle() -> std::io::Result<()>{
		let sock_addrs = [
			SocketAddr::from(([127.0.0.1], 80)),
			SocketAddr::from(([127.0.0.1], 443)),
			SocketAddr::from(([127.0.0.1], 3000)),
		];
	
		let listener = TcpListener::bind(&sock_addrs[..]).unwrap()?;
	
		for tcps in listener.incoming() {
			Handler_t::handle_client(tcps: TcpStream?);
	
			match tcps {
				Ok(tcps) => {
					println!("Client - NEW: {:?}", tcps);
				}
				Err(e) => println!("Couldn't get client (with handler): {:?}", e),
			}//match tcps
		}//for tcps in listener.incoming()
	}//fn client_main_handle()
}//impl<'a> Handler_t