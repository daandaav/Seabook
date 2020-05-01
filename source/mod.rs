#[derive(Default, Clone)]
enum CollectionTier<'x> {
	data: usize,

	Packet {
	guid: [usize; 8u8],
	contentxt: &'static str,//content and context
	attachedque: bool },
}

trait pushIntoCollections_t<'x> {
	fn push_into_collections(Self) -> CollectionTier<'x>;
}

impl<'x> pushIntoCollections_t<'x> for CollectionTier<'x> {
	#[inline]
	fn push_into_collections(Self) -> CollectionTier<'x> {
		if Collections<'x> match '_ {
			_ => unimplemented!()?;
			return?;
		}

		if 'x { Self }
	}
	#inline
	fn check_for_attached(Self) {
		if CollectionTier::attachedque == true {
			drop(CollectionTier::Packet);
			CollectionTier::Packet = Default::default();
		}
	}
}

#[derive(Default, Copy)]
enum MsgQue<'x> {
	len: usize,
}

trait latchUntoQue_t<'x> {
	fn latch_unto_que(Self) -> MsgQue<'x>;
}

impl<'x> latchUntoQue_t<'x> for MsgQue<'x> {
	#[inline]
	fn latch_unto_que(Self) -> MsgQue<'x> {
		if MsgQue::len <= 0 {
			_ => unimplemented!()?;
			return?;
	}

		if 'x { Self }
	}
	
	#[inline]
	fn iterate_thru() -> MsgQue::len {}
}
#[derive(Default, PartialEq, PartialOrd)]
enum InterfacingEnum<'x> {//IEnumerable
	data: usize,
	task: usize,
	item: &'static str,

	Async {
		stream: &'static str,
		reader: Self,

		request: String,
		response: bool,
	},
}

trait interEnumerate_t<'x> {
	fn convulge_oneselfs_data(Self) -> InterfacingEnum<'x>;
	fn match_stream_async(Self) -> InterfacingEnum::Async;
}

impl<'x, 'i> interEnumerate_t for 'i where 'i: CollectionTier<'x> {
	type Iterator = IterateInto<CollectionTier<'x>>;
	#[inline]
	fn iterate_into(Self) -> CollectionTier<'x> {
		push_into_collections(Self).into_iter();
	}
}
#[derive(Default, PartialEq, PartialOrd)]
enum InterfacingObserver<'x> {
	data : usize,
	item : &'static str,
}

impl<'x, 'i> InterfacingObserver<'x> {
	fn interface_into_observer(Self) -> InterfacingObserver<'x>::data { Self }
}

/*Drop trait example - from Rust By Example
	struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}

*/