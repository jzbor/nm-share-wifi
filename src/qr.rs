use fast_qr::qr::QRBuilder;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};

pub struct QRCode {
    code: fast_qr::qr::QRCode,
}

impl QRCode {
    pub fn from_str(contents: &str) -> Result<Self, String> {
        let code = QRBuilder::new(contents).build()
            .map_err(|_| format!("unable to build QR code"))?;

        Ok(QRCode { code })
    }

    pub fn to_ascii(&self) -> String {
        self.code.to_str()
    }

    pub fn to_pixels(&self) -> Vec<u8> {
        ImageBuilder::default()
            .shape(Shape::RoundedSquare)
            .background_color([255, 255, 255, 0]) // transparency
            .fit_width(600)
            .to_pixmap(&self.code)
            .take()
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), String> {
        ImageBuilder::default()
            .shape(Shape::RoundedSquare)
            .background_color([255, 255, 255, 0]) // transparency
            .fit_width(600)
            .to_file(&self.code, path)
            .map_err(|_| format!("unable to save qr code to file"))
    }
}
