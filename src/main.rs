use rkyv::{Archive, Serialize};
use subcrate::{GenericObj, Const};

#[derive(Archive, Serialize)]
pub enum Enum {
	None,
	Contains(GenericObj<{Const{}}>), // Doesn't error without this
}

fn main() {
    let mut serializer = rkyv::ser::serializers::AllocSerializer::<256>::default();
	(&Enum::None).serialize(&mut serializer).unwrap();
}