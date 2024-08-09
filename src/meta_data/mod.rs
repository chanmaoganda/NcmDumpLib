mod image;
mod metadata;
mod mp3_metadata;
mod flac_metadata;

use image::{guess_format, ImageFormat};
pub use metadata::{ MetaData, ncm_metadata_builder };
pub use mp3_metadata::Mp3MetaData;
pub use flac_metadata::FlacMetadata;

pub fn guess_pict_type(data: &[u8]) -> String {
    let format = guess_format(data).unwrap_or(ImageFormat::Jpeg);
    match format {
        ImageFormat::Png => "png".to_owned(),
        ImageFormat::Jpeg => "jpeg".to_owned(),
        ImageFormat::Bmp => "bmp".to_owned(),
        ImageFormat::Ico => "ico".to_owned(),
        _ => "unknown".to_owned(),
    }
}