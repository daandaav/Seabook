use std::prelude::v1::*;

#[derive(Debug, Copy)]
enum Lexicon<T, S> {
	track(T),
	state(S),
}

/*
	Memory with Bitmap and Array combo program:
	TODO(NoExtern, Buffer/Stream): Have memory work without an external 'C' program.
		Bitmap.rs should track null values in an array. We're reworking this into tracker for persistent states.
	
	The Lexicon works like Python's Dictionary, and generic array/list. Although this will be where we track and reserve persistent states \
		which are the lossless and reliable-of-fault subsystem, should the main system's connection be severed or fail.

	Our Lexicon should derive from either a hashmap function or a vector function.
*/

trait withinLexiconTrackState<T, S> -> Result<()> {//Result with the caret <> could either be: just as a caret <>:;
	//with template <T>:;
	//or with function parameter <()>	//a careted function parameter is idead for I/O with the Result
	fn track_value(Self) -> Result<()>;
	fn record_state(Self) -> Vec<S>;
}
//impl<T : Read> or impl<T : BufReader> [?]
impl<T> withinLexiconTrackState for Result<()> where T : Lexicon<T> {
	fn track_value(Self) -> Result<()> {
		match Self {
			Lexicon::track(ref Self) => Self.iter(Result<(T)>),
			'_ => unimplemented!("Resulted in a Wildcard[?!]: {}\n", Lexicon::track(Self)),
		}

		if Self == '_ {
			panic!("{Lexicon::track(ref Self)} => Self.iter(Result<(T)>)\n ...has resulted into [[panic!()]]")?;
			Err(T)
		}//if Self == _
	}//fn track_value(Self)

	fn record_state(Self) -> Vec<S> {
		match Self {
			Lexicon::state(ref Self) => Vec<Self>
		}

		if Self == -1 {
			panic!("{Lexicon::state(Self)} => Vec<Self>", Lexicon::state(Self))?,
			Err(S)
		}//if Self == -1
	}
}//impl<T> withinLexiconTrackValue

#[derive(Clone, PartialEq)]
pub struct State {
	id : u32,

	broker : Vec<usize>,

	msg : Hash<String, Referee>
}

pub struct Producer {
	id : u32,
	agents : Vec<Broker>,
}
//...
impl<T> Producer for State {
	fn pair<T>(Self, s : T) {
		Self.id.hash(s);
	}
}

#[derive(PartialEq)]
pub struct Broker {
	id : i32,

	Message {
		len : usize,
		msg : &'static str,
	}
}
//...
impl<T> Broker for Producer {
	let v = Vec::new();

	fn produce<T>(Self, p : T) {
		Self.id.hash(p);
		Self.v.push(p);
	}
}

pub struct Referee<T> {
	agents : Vec<T>,
}

impl<T> Referee<T> for Broker -> Result<()> {
	pub fn check_broker_index(self : &Self, f : &mut fmt::Formatter<'_>) -> Result {
		write!("{Self.Broker::id}",
			self.Broker::id)
	}

	pub fn refer_to_broker<usize, T>(&Self) {
		assert_eq!((usize::max_value(ref Self), None), T.size_hint())
	}
}

pub struct Kollective<T> {
	referal : Cow<Referee>,
	iter_dex : iter::<Iterator<'a>>,
}

pub trait iterIntoAgentsList_t<'a> {
	fn agent_iter_into(&self) -> Option<Self::Agent>;//TODO(Iterator):...create hi-func iterator -> here
}

pub impl<T:'a> iterIntoAgentsList<'a> for Result<usize, ()> where T: Kollective {
	//...
	type Agent = (&'a Kollective);

	fn agent_iter_into(&self) -> Option<Self::Agent> {
		self.iter.next()//\
			.map(|hm| &hm.referal)
	}
}

pub struct Consumer {}
//...
pub struct Client {
	bloc : Cow<usize>,
}

impl<'a> Client {

	pub fn seek_available_brokers(&self) {
		self.Lexicon.track(agents)
			.state(|hm| &hm.referal)
	}//TODO(Iterator):...need to create a higher-fumction iterator

	pub fn iter_brokers_cow(&self) {
		//...
		for i in Broker.iter() {
			//...push and map to Lexicon
			let mut v = Vec::new();

			let Broker::id<&self, T : i32> = v.push(T as usize);
			//let 'a = Client::bloc.iter()
			//TODO: if-else statement for object lifetime
			if T > i { panic!(), Err(()) }
		}
	}//fn iter_broke_cow(self, a)
}