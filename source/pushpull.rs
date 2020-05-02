use std::prelude::v1::*;

#[derive(Debug, Copy)]
enum Data<T> {
	data(T),
}

enum Push<T> {
	data : usize,

	push(T),
}

impl<T : Read> Push for Data -> Result<usize> {
	fn data_push_into_iter(Self) -> Result<usize> {
		match Self {
			Data::data(ref Self) => Self.read()
	}
}

enum Pull<T> {
	data : usize,

	pull(T),
}

impl<T : Drop> Pull for Data -> Result<usize> {
	fn data_pull_from_iter(Self) -> Result<usize> {
		match Self {
			Data::data(ref Self) => Self.drop()
		}
	}
}