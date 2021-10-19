#[macro_export]
macro_rules! endpoint {
    ($uri: expr) => {
        if cfg!(debug_assertions) {
            format!("http://0.0.0.0:7878{}", $uri)
        } else {
            format!("https://estebanborai.herokuapp.com{}", $uri)
        }
    };
}
