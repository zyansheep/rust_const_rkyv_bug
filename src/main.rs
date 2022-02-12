use subcrate::*;

fn main() {
	<Foo::<{Const{}}> as Bar>::associated();
}