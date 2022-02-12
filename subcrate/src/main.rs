use rkyv::Serialize;
use subcrate::{GenericObj, Const};

// This doesn't error, but in the main crate it does?
fn main() {
    let mut serializer = rkyv::ser::serializers::AllocSerializer::<256>::default();
	GenericObj::<{Const{}}>::default().serialize(&mut serializer).unwrap();
}