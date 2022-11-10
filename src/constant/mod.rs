use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLUSTER_ENABLE: bool = {
        match std::env::var("CLUSTER_ENABLE") {
            Ok(enable) => {
                if enable == "true" {
                    true
                } else {
                    false
                }
            }
            Err(..) => {
                false
            }
        }
    };
    pub static ref REDIS_URL: String = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    pub static ref REDIS_CLUSTER_URL: String = std::env::var("REDIS_CLUSTER_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
}
