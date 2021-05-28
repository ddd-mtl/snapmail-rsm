

#[cfg(target_arch = "wasm32")]
fn main() {
 // Dummy for wasm32 target
}

#[cfg(not(target_arch = "wasm32"))]
pub mod test;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
   crate::test::test().await;
}
