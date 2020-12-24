use serde::Serialize;

pub trait Model<T: Serialize> {
    fn setup(&self);
    fn go(&self) -> T;
}
