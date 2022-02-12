#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
pub struct Const {}
impl Const {
	pub const fn func(self) -> usize { 1 }
}

pub struct Foo<const C: Const>
where
	[(); C.func()]: Sized
{}

pub trait Bar {
	type Associated;
	fn associated() -> Self::Associated;
}

impl<const C: Const> Bar for Foo<C>
where
	[(); C.func()]: Sized
{
	type Associated = [(); C.func()];
	fn associated() -> Self::Associated {
		[(); C.func()]
	}
}