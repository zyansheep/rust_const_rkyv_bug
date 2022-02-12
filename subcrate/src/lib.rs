#![feature(generic_const_exprs)]
#![feature(adt_const_params)]


#[derive(PartialEq, Eq)]
pub struct Const {}
impl Const {
	pub const fn func(self) -> usize { 1 }
}

pub trait Test {
	type Associated;
	fn ret() -> Self::Associated;
}

pub struct GenericObj<const C: Const>
where
	[(); C.func()]: Sized
{}

impl<const C: Const> Test for GenericObj<C>
where
	[(); C.func()]: Sized
{
	type Associated = [(); C.func()];
	fn ret() -> Self::Associated {
		[(); C.func()]
	}
}