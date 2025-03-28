#[macro_export]
macro_rules! generate_response {
    ($code:expr, $msg:expr) => {
        axum::http::Response::builder()
            .status($code)
            .body(Body::from($msg))
            .expect("Failed to statically generate response")
      }
}
