use thiserror::Error;

// 定义一个枚举来表示可能的错误
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Input/Output error")]
    Io(#[from] std::io::Error),
    #[error("The API interface request failed")]
    Api(#[from] reqwest::Error),
    #[error("Not found: {}", reason)]
    NotFound { reason: String },
}
