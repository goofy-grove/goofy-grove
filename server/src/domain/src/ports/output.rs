pub trait IdGenerator {
    fn generate(&self) -> String;
}
