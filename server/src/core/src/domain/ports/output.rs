pub trait IdGenerator {
    fn generate(&self) -> String;
}

pub trait Clock {
    fn timestamp(&self) -> i64;
}
