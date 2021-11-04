// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
struct Server;

impl Operand for Server {
    // Each defined rpc generates two items in the trait, a fn that serves the RPC, and
    // an associated type representing the future output by the fn.

    type Fut = Ready<String>;

    fn summ(self, _: context::Context, digits: String) -> Self::Fut {
        let mut ws = digits.trim().split(" ");
        let mut summ = 0.0;
        for i in ws
        {
            summ += i as f32;
        }
        future::ready(format!("summ = {}", summ))
    }
}
