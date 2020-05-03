use std::prelude::v1::*;

#[derive(Debug, Copy)]
enum Lexicon<T> {
	len : usize,
	state : &'static str//explicit type annotation of a string-slice
	track(T),
}

/*
	Memory with Bitmap and Array combo program:
	TODO(NoExtern, Buffer/Stream): Have memory work without an external 'C' program.
		Bitmap.rs should track null values in an array. We're reworking this into tracker for persistent states.
	
	The Lexicon works like Python's Dictionary, and generic array/list. Although this will be where we track and reserve persistent states \
		which are the lossless and reliable-of-fault subsystem, should the main system's connection be severed or fail.

	Our Lexicon should derive from either a hashmap function or a vector function.
*/

trait withinLexiconTrackValue<T> -> Result<()> {//Result with the caret <> could either be: just as a caret <>:;
	//with template <T>:;
	//or with function parameter <()>	//a careted function parameter is idead for I/O with the Result
	fn track_value(Self) -> Result<()>;
}
//impl<T : Read> or impl<T : BufReader> [?]
impl<T> withinLexiconTrackValue for Lexicon where T : Result<()> {
	fn track_value(Self) -> Result<()> {
		match Self {
			Lexicon::track(ref Self) => Self.iter(Result<(T)>),
			_ => unimplemented!("Resulted in a Wildcard[?!]: {}\n", Lexicon::track(Self)),
		}

		if Self == 0 | _ {
			panic!("{Lexicon::track(ref Self)} => Self.iter(Result<(T)>)\n ...has resulted into [[panic!()]]")?;
			return;
			Err(T)
		}//if Self == 0 | _
	}//fn track_value(Self)
}//impl<T> withinLexiconTrackValue