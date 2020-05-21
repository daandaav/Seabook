pub enum Moc<T> {
	moc(T)
}

trait MocTestT {
	fn test(&self) -> Result<()>;
}

pub impl<T> MocTestT for Result<()> where T: Moc {
	#[test]
	fn test(&self) -> Result<()> {
		if self
		{
		//...
			match self {
				Moc::moc(ref self) => S.iter_into(self);
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
				assert_eq!(expected, test(&self)),
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