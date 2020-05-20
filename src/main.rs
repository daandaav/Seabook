mod coll;
mod event;

pub fn main() {
	println!("Seabook objects and their attributes should be running here...\nHmmm...");

	coll::handle_client();
	coll::client_main_handle();

	event::request_input();
	event::pend_for_data();
}
