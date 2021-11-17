#[tarpc::service]
pub trait Operand {
    /// Returns a sum of items in string.
    async fn add(items: String) -> String;
}