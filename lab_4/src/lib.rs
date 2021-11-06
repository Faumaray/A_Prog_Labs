#[tarpc::service]
pub trait Operand {
    /// Returns a greeting for name.
    async fn add(items: String) -> String;
}