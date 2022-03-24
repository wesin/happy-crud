use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use anyhow::Result;
use futures::{Stream, StreamExt};
use log::*;
use tokio::fs;
use warp::{http::HeaderValue, Reply};
use zip_extensions::zip_extract;

use crate::{
    config::get_config,
    model::file::FileUploadDO,
    utils::{ws_error::WSError, ws_response::response_empty},
};

fn file_path() -> &'static str {
    &get_config().bundle.path
}

pub async fn download_file(
    args: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let bundle_id = args.get("bundle_id");
    if bundle_id.is_none() {
        return Ok(WSError::RequestParamError("bundle_id".to_string()).into_response());
    }
    let version_id = args.get("version_id");
    if version_id.is_none() {
        return Ok(WSError::RequestParamError("version_id".to_string()).into_response());
    }
    let path = Path::new(file_path())
        .join(bundle_id.unwrap())
        .join(version_id.unwrap().to_owned() + ".zip");
    if !path.exists() {
        return Ok(WSError::Error(110, "bundle not found".to_string()).into_response());
    }
    let mut file = std::fs::File::open(path).map_err(WSError::FileError)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).map_err(WSError::FileError)?;
    let mut res = warp::reply::Response::new(data.into());
    res.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_static("application/x-zip-compressed"),
    );
    Ok(res)
}

pub async fn handle_upload_file<S, B>(
    upload_do: FileUploadDO,
    stream: S,
) -> Result<impl warp::Reply, warp::Rejection>
where
    S: Stream<Item = Result<B, warp::Error>>,
    S: StreamExt,
    B: warp::Buf,
{
    let path = Path::new(file_path()).join(&upload_do.bundle_id);
    if !path.exists() {
        fs::create_dir_all(&path)
            .await
            .map_err(WSError::FileError)?;
    }
    let file_path = Path::new(file_path())
        .join(&upload_do.bundle_id)
        .join((&upload_do.version_id).to_owned() + ".zip");
    info!("begin save file");
    let mut file = File::create(&file_path).map_err(WSError::FileError)?;
    let mut pinned_stream = Box::pin(stream);
    while let Some(item) = pinned_stream.next().await {
        let data = item.unwrap();
        file.write_all(data.chunk()).unwrap();
    }
    info!("begin md5");
    let mut dd = Vec::new();
    File::open(&file_path)
        .map_err(WSError::FileError)?
        .read_to_end(&mut dd)
        .map_err(WSError::FileError)?;
    let digest = md5::compute(&dd);
    info!("md5:{:x}", &digest);
    let md5_value = format!("{:x}", digest);
    if upload_do.md5.to_uppercase() != md5_value.to_uppercase() {
        return Ok(WSError::Error(110, "md5 not equal".to_string()).into_response());
    }
    let file_dir = path.join(&upload_do.bundle_id);
    zip_extract(&file_path, &file_dir).unwrap();
    Ok(response_empty().into_response())
}
