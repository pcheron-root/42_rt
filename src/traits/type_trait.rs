use std::any::Any;
use std::any::TypeId;

pub trait TypeTrait {
    fn is_tuple_type(&self) -> bool;
}

impl<T: Any> TypeTrait for T {
    fn is_tuple_type(&self) -> bool {
        self.type_id() == TypeId::of::<(i32, i32)>()
    }
}
