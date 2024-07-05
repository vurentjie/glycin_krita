// This mostly a stripped down version of the loader in the 1.0.1 tag of glycin source repo
// Found here: https://gitlab.gnome.org/sophie-h/glycin/-/blob/1.0.1/loaders/glycin-image-rs/src/bin/glycin-image-rs.rs?ref_type=tags
// I haved kept parts related to loading png images and added support to extract "mergedimage.png" from the .kra or .ora file.

#![allow(clippy::large_enum_variant)]

use std::io::{Cursor, Read};
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

use glycin_utils::image_rs::Handler;
use glycin_utils::*;
use image::io::Limits;
use image::{codecs, ImageDecoder, ImageResult};

init_main!(ImgDecoder::default());

type Reader = Cursor<Vec<u8>>;

#[derive(Default)]
pub struct ImgDecoder {
    pub format: Mutex<Option<ImageRsFormat<Reader>>>,
    pub thread: Mutex<Option<(std::thread::JoinHandle<()>, Receiver<Frame>)>>,
}

impl LoaderImplementation for ImgDecoder {
    fn init(
        &self,
        mut stream: UnixStream,
        mime_type: String,
        _details: InitializationDetails,
    ) -> Result<ImageInfo, LoaderError> {
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).internal_error()?;

        let archive_data = Cursor::new(buf);
        let mut archive = zip::ZipArchive::new(archive_data).unwrap();
        let mut file = match archive.by_name("mergedimage.png") {
            Ok(file) => file,
            Err(..) => {
                return Err(LoaderError::UnsupportedImageFormat(String::from(
                    "Failed to load file",
                )));
            }
        };

        let mut merged_image = Vec::new();
        file.read_to_end(&mut merged_image).internal_error()?;

        let image_data = Cursor::new(merged_image);
        let mut format = ImageRsFormat::create(image_data.clone(), &String::from("image/png"))?;
        if let Err(err) = format.set_no_limits() {
            eprint!("Failed to unset decoder limits: {err}");
        }

        let mut image_info = format.info();
        let exif = exif::Reader::new().read_from_container(&mut image_data.clone());
        let format_name = match mime_type.as_str() {
            "image/openraster" => "OpenRaster Image",
            "application/x-krita" => "Krita document",
            _ => "Unknown",
        };

        image_info.details.exif = exif.ok().map(|x| BinaryData::from(x.buf().to_vec()));
        image_info.details.format_name = Some(format_name.to_string());
        *self.format.lock().unwrap() = Some(format);

        Ok(image_info)
    }

    fn frame(&self, _frame_request: FrameRequest) -> Result<Frame, LoaderError> {
        let frame = if let Some(decoder) = std::mem::take(&mut *self.format.lock().unwrap()) {
            decoder.frame().loading_error()?
        } else if let Some((ref thread, ref recv)) = *self.thread.lock().unwrap() {
            thread.thread().unpark();
            recv.recv().unwrap()
        } else {
            unreachable!()
        };

        Ok(frame)
    }
}

pub enum ImageRsDecoder<T: std::io::Read + std::io::Seek> {
    Png(codecs::png::PngDecoder<T>),
}

pub struct ImageRsFormat<T: std::io::Read + std::io::Seek> {
    decoder: ImageRsDecoder<T>,
    handler: Handler,
}

impl ImageRsFormat<Reader> {
    fn create(data: Reader, mime_type: &str) -> Result<Self, LoaderError> {
        Ok(match mime_type {
            "image/png" => Self::new(ImageRsDecoder::Png(
                codecs::png::PngDecoder::new(data).loading_error()?,
            ))
            .default_bit_depth(8),
            mime_type => return Err(LoaderError::UnsupportedImageFormat(mime_type.to_string())),
        })
    }
}

impl<'a, T: std::io::Read + std::io::Seek + 'a> ImageRsFormat<T> {
    pub fn default_bit_depth(mut self, default_bit_depth: u8) -> Self {
        self.handler = self.handler.default_bit_depth(default_bit_depth);
        self
    }

    fn new(decoder: ImageRsDecoder<T>) -> Self {
        Self {
            decoder,
            handler: Handler::default(),
        }
    }

    fn info(&mut self) -> ImageInfo {
        match self.decoder {
            ImageRsDecoder::Png(ref mut d) => self.handler.info(d),
        }
    }

    fn frame(self) -> Result<Frame, LoaderError> {
        match self.decoder {
            ImageRsDecoder::Png(d) => self.handler.frame(d),
        }
    }

    fn set_no_limits(&mut self) -> ImageResult<()> {
        match self.decoder {
            ImageRsDecoder::Png(ref mut d) => d.set_limits(Limits::no_limits()),
        }
    }
}

