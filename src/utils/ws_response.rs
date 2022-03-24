use serde::Serialize;
use warp::{reply::Response, Reply};

#[derive(Serialize)]
pub struct WSResponse<T>
where
    T: serde::Serialize,
{
    code: u16,
    data: Option<T>,
    error: Option<String>,
}

impl<T> WSResponse<T>
where
    T: serde::Serialize,
{
    pub fn result(code: u16, data: Option<T>) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            code,
            data,
            error: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            code: 1,
            data: None,
            error: None,
        }
    }

    pub fn data(data: T) -> Self
    where
        T: serde::Serialize,
    {
        Self {
            code: 1,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: u16, error: String) -> Self {
        Self {
            code,
            data: None,
            error: Some(error),
        }
    }
}

impl<T> Reply for WSResponse<T>
where
    T: serde::Serialize + std::marker::Send,
{
    fn into_response(self) -> Response {
        warp::reply::json(&self).into_response()
    }
}

pub fn response_empty() -> Response {
    warp::reply::json(&WSResponse::<String>::empty()).into_response()
}
