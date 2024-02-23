use data_url::DataUrl;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataUrlImageError {
    #[error("Data URL is not an image")]
    InvalidMimeType,
}

pub struct DataUrlResponse {
    pub mimetype: String,
    pub data: Vec<u8>,
}

pub fn process_data_url_image(
    image_url: &str,
) -> Option<Result<DataUrlResponse, DataUrlImageError>> {
    let processed = DataUrl::process(image_url).ok()?;

    if processed.mime_type().type_ != "image" {
        return Some(Err(DataUrlImageError::InvalidMimeType));
    }

    let (bytes, _) = processed.decode_to_vec().ok()?;

    Some(Ok(DataUrlResponse {
        mimetype: processed.mime_type().to_string(),
        data: bytes,
    }))
}
