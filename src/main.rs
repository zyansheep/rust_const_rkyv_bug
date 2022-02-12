use subcrate::*;

fn main() {
	<GenericObj::<{Const{}}> as Test>::ret();
}