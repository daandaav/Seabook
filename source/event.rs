enum Poll<T> {
	Ready(T),
	Pending,
}

struct ReadIntoFuture<&'a> {
	input_read : &'a Socket,
}

trait PollInForFuture_t {
	type Output;

	fn poll_out(
		self : Pin<&Self>.

		cx : &Context<'_>,
	) -> Poll<Self::Output>;
}

trait ReadoutInFuture_t {
	type Output;

	fn read_out_poll(&self, wake_n_make: fn()) -> Poll<Self::Output>;
}

impl<'a> ReadoutInFuture_t {
	type Input = Poll<&'a>;

	fn request_for_input(&self) -> Poll<Self::Input>;

	fn read_out_poll(&self, wake_n_make: fn()) -> Poll<Self::Output> {
		if self.input_read.has_data_to_read() {
			Poll::Ready(self.input_read.read_buf())
		} else {
			self.input_read.set_readable_callback(wake_n_make);
			Poll::Pending
		}
	}
}

struct OpForItemChoice<It> {
	item_choice : Option<It>,
}

impl<It> ReadOutIntoFuture for OpForItemChoice<It> where It : ReadIntoFuture<Output = ()> {
	type Output = ();
	
	fn poll_out(&self, wake_n_make: fn()) -> Poll<Self::Output> {
		if let Some(a) = self.a {
			if let Poll::Ready(()) = a.poll_out(wake_n_make) {
				self.a.take();
			}
		}

		if self.a.is_none() {
			Poll::Ready(())
		} else {
			Poll::Pending
		}
	}
}

/*
	Asynchronous Programming in Rust:
	https://rust-lang.github.io/async-book/02_execution/02_future.html
*/