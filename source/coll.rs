use {
	std::net::{
		SocketAddr,
		TcpListener,
		TcpStream,
	},
	std::prelude:v1::*,
};

enum Collection<T> {
	Data(T),
	Await,

	Pointer {
		//...iterate through a collection
		dataset_alloc : Vec<u32>,
		iter_thru(T),
	},
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
					.cycle() } else if i > d.iter() {
						b.clear();
						break;
					} //TODO: b.clear()
		}//fn man_buffer_size
}

trait Handler_t {
	fn handle_client(s : TcpStream);

	fn client_main_handle() -> std::io::Result<()>;
}

impl Handler_t {
	fn handle_client(s : TcpStream) {
		//TODO(Client Handler): ...
		
	}

	fn client_main_handle() -> std::io::Result<()>{
		let sock_addrs = [
			SocketAddr::from(([127.0.0.1], 80)),
			SocketAddr::from(([127.0.0.1], 443)),
			SocketAddr::from(([127.0.0.1], 3000)),
		];
	
		let listener = TcpListener::bind(&sock_addrs[..]).unwrap()?;
	
		for s in listener.incoming() {
			Handler_t::handle_client(s: TcpStream?);
	
			match s {
				Ok(s) => {
					println!("Client - NEW: {:?}", s);
				}
				Err(e) => println!("Couldn't get client (with handler): {:?}",e),
			}
		}
	}
}