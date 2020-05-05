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
impl<T> withinLexiconTrackState for Lexicon where T : Result<()> {
	fn track_value(Self) -> Result<()> {
		match Self {
			Lexicon::track(ref Self) => Self.iter(Result<(T)>),
			_ => unimplemented!("Resulted in a Wildcard[?!]: {}\n", Lexicon::track(Self)),
		}

		if Self == _ {
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