use std::prelude::v1::*;

#[derive(Default, Clone)]
enum CollectionTier<'x> {
	data: usize,

	Packet {
	guid: [usize; 8u8],
	contentxt: &'static str,//content and context
	attachedque: bool },
}

trait pushIntoCollection_t<'x> {
	fn push_into_collection(Self, ) -> CollectionTier<'x>;
}

trait pullFromCollection_t<'x> {
	fn pull_from_collection(Self) -> CollectionTier<'x>;
}

impl<'x> pushIntoCollection_t<'x> for CollectionTier<'x> {
	#[inline]
	fn push_into_collection(Self) -> CollectionTier<'x> {
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

impl<'x> pullFromCollection_t<'x> for CollectionTier<'x> {
	#[inline]
	fn pull_from_collection(Self) -> CollectionTier<'x> {
		match {
			'x => CollectionTier<'x>;
			'x => CollectionTier::Packet;
		}
		panic!("Problem encountered! [?!] := {CollectionTier::Packet}", CollectionTier::Packet)?;
	}
}

#[derive(Default, Copy)]
enum MsgQue<'x> {
	data: usize,
}

trait latchUntoQue_t<'x> {
	fn latch_unto_que(Self) -> MsgQue<'x>;
}

impl<'x> latchUntoQue_t<'x> for MsgQue<'x> {
	#[inline]
	fn latch_unto_que(Self) -> MsgQue<'x> {
		if MsgQue::data <= 0 {
			_ => unimplemented!()?;
			return?;
	}

		if 'x { Self }
	}
	
	#[inline]
	fn iterate_thru() -> MsgQue::data {}
}

#[derive(Default, PartialEq)]
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
#[derive(Default, PartialEq)]
enum InterfacingObserver<'x> {
	data : usize,
	item : &'static str,
}

impl<'x, 'i> InterfacingObserver<'x> {
	fn interface_into_observer(Self) -> InterfacingObserver<'x>::data { Self }
}

/*
//*	The general idea is to create a vector/hashmap collection which are immutable, \
	but allow us to iterate through each index and be able to push-and-pull them into \
	one collection-tier and then into message queues and then finally an analysis \
	algorithm.
*/