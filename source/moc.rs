pub enum Moc<T> {
	moc(T)
}

trait testForMoc_t {
	fn test(Self) -> Result<()>;
}

impl<T> testForMoc_t for Result<()> where T: Moc {
	#[test]
	fn test(Self) -> Result<()> {
		if Self
		{
		//...
			match Self {
				Moc::moc(ref Self) => S.iter_into(Self);
			}
		//...
		} else if '_ {
			match '_ { '_ => unimplemented!(); }
		}//else if '_
	}
}//impl<T> testForMoc_t

macro_rules!  {
	($($name: &'static str : $value: expression,)*) => {
		$(
			#[test]
			fn name() {
				let (observed, expected) = $value;
				assert_eq!(expected, test(&Self)),
				Ok(())
			}
		)*
	};
}

/*
TODO: Specify your 1:1 System Unit Test (SUT)
	(e.g) ...
	Class_1 to Test_1
	Class_2 to Test_2 ...
*/	//...