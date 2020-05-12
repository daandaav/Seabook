use std::prelude:v1::*;

enum Collection<T> {
	Data(T),
	Await,

	Cursor {
		//...iterate through a collection
		dataset_alloc : Vec<usize>,
		iter_thru(T),
	},
}

struct Message {
	data_collection : Vec<Collection>,

	buffer : i32,

	read_only : Arc<Collection>,
}

trait InterMsgQ_t {
	type Msg;

	fn read_interface_msg(self : Collection<&self>, m : &Message::read_only<Self::Msg>);

	fn man_buffer_size(&self, b : &Message::buffer);
}

impl<'a> InterMsgQ_t {
	type Msg = Collection<&'a>;

		fn read_interfacing_msg(
			self : Collection<&self>,
			m : &Message::read_only,
		) {
			if (let i = 0u32) < self.iter() {
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
				self.iter(d)
					.map(|h| h.buffer)
					.cycle() } else if i > d.iter() { return?; }
		}//fn man_buffer_size
}