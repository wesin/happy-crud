use std::convert::Infallible;

use anyhow::Result;
use log::error;
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;
use warp::{http::*, reject, Rejection, Reply};
use zip::result::ZipError;

#[derive(Debug, Error)]
pub enum WSError {
    #[error("{1}")]
    Error(u16, String),
    #[error("param is must: [{}]", .0)]
    RequestParamError(String),
    #[error("sql error: [{}]", .0)]
    SqlError(DbErr),
    #[error("file error: [{}]", .0)]
    FileError(std::io::Error),
    #[error("json error: [{0}]")]
    JsonError(serde_json::error::Error),
    #[error("zip error: [{0}]")]
    ZIPError(ZipError),
}

impl WSError {
    fn status_code(&self) -> StatusCode {
        match self {
            WSError::RequestParamError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::OK,
        }
    }

    fn code(&self) -> u16 {
        match self {
            WSError::RequestParamError(_) => 110,
            WSError::Error(code, _) => code.clone(),
            WSError::SqlError(_) => 1100,
            WSError::FileError(_) => 1200,
            WSError::ZIPError(_) => 1201,
            WSError::JsonError(_) => 1202,
        }
    }
}

#[derive(Debug, Serialize)]
struct ResponseError {
    code: u16,
    error: String,
}

impl Reply for WSError {
    fn into_response(self) -> warp::reply::Response {
        Response::builder()
            .status(self.status_code())
            .body(
                serde_json::to_string(&ResponseError {
                    code: self.code(),
                    error: format!("{}", self),
                })
                .unwrap()
                .into(),
            )
            .unwrap()
    }
}

impl reject::Reject for WSError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(e) = err.find::<WSError>() {
        let berr = WSError::Error(e.code(), format!("{:?}", err));
        Ok(berr.into_response())
    } else {
        let berr = WSError::Error(500, format!("{:?}", err));
        Ok(berr.into_response())
    }
}
