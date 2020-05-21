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
	fn write_copy(&self, rwc : Vec<usize>);

	fn read_copy(&self, rwc : Vec<usize>);
}

impl<'a> dyn CollectionT {
	//...
	fn write_copy(&self, b : usize,rwc : Vec<usize>) {
		match self {
			r => rwc = Vec::new(),
		}
			let r = rwc;

			r.iter()
				.clone()
				.map(|hm| b)
				.next();

			if b > r.len() {
				panic!("Buffer is greater than Read-Write-Copy size!")
			}
	}
	
	fn read_copy(&self, b : usize, rwc : Vec<usize>) {
		match self {
			r => rwc = Vec::new(),
		}//match self

		let r = rwc;

		//println!("{}", r.iter());//implement fmt::Display...
		//...
	}//fn read_copy(&self)
}

struct Message {
	data_collection : usize,

	buffer : Vec<usize>,

	read_only : Rc<usize>,
}

trait InterMsgQT {
	fn read_interface_msg(&self, c : Vec<usize>, m : usize);

	fn man_buffer_size(&self, d : Vec<usize>, b : usize);
}

impl<'a> dyn InterMsgQT {
		fn read_interfacing_msg(
			&self,
			c : Vec<usize>,
			m : usize,
		) {
			let capacity;
			
			c = Vec::with_capacity(capacity: usize);
			//...
		}//fn read_interfacing_msg

		fn man_buffer_size(
			&self,
			d : Vec<usize>,
			b : usize,
		) {
			//TODO(BUffer): Map d_vec<usize> iterations to b_for_buffer:usize
		}//fn man_buffer_size
}

trait HandlerT {
	fn client_main_handle(&self);
}

impl dyn HandlerT {

	fn client_main_handle(&self) {
		let sock_addrs = [
			SocketAddr::from(([127,0,0,1], 80)),
			SocketAddr::from(([127,0,0,1], 443)),
			SocketAddr::from(([127,0,0,1], 3000)),
		];
	
		let listener = TcpListener::bind(&sock_addrs[..]).unwrap();
	
		for tcps in listener.incoming() {
			match tcps {
				Ok(tcps) => {
					println!("Client - NEW: {:?}", tcps);
				}
				Err(e) => println!("Couldn't get client (with handler): {:?}", e),
			}//match tcps
		}//for tcps in listener.incoming()
	}//fn client_main_handle()
}//impl<'a> HandlerT