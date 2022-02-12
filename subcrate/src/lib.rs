#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

use rkyv::{Archive, ser::{ScratchSpace, Serializer}};

#[derive(PartialEq, Eq)]
pub struct Const {}
impl Const {
	pub const fn func(self) -> usize { 1 }
}
pub struct GenericObj<const C: Const>
where
	[(); C.func()]: Sized
{}
impl<const C: Const> Archive for GenericObj<C>
where
	[(); C.func()]: Sized
{
	type Archived = GenericObj<C>;
	type Resolver = [(); C.func()];

	unsafe fn resolve(&self, _pos: usize, _resolver: Self::Resolver, _out: *mut Self::Archived) {}
}

impl<const C: Const, S: ScratchSpace + Serializer + ?Sized> rkyv::Serialize<S> for GenericObj<C>
where
	[(); C.func()]: Sized
{
	fn serialize(&self, _serializer: &mut S) -> Result<Self::Resolver, S::Error> {
		Ok([(); C.func()])
	}
}