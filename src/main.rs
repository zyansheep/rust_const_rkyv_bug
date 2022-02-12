use rkyv::Serialize;
use subcrate::{GenericObj, Const};

fn main() {
    let mut serializer = rkyv::ser::serializers::AllocSerializer::<256>::default();
	GenericObj::<{Const{}}>::default().serialize(&mut serializer).unwrap();
}