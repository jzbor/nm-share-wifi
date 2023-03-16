use fast_qr::qr::QRBuilder;
use fast_qr::convert::{image::ImageBuilder, Builder, Shape};

pub struct QRCode {
    code: fast_qr::qr::QRCode,
}

const PIXBUF_WIDTH: u32 = 600;

impl QRCode {
    pub fn from_str(contents: &str) -> Result<Self, String> {
        let code = QRBuilder::new(contents).build()
            .map_err(|_| format!("unable to build QR code"))?;

        Ok(QRCode { code })
    }

    pub fn to_ascii(&self) -> String {
        self.code.to_str()
    }

    pub fn to_gdk_pixbuf(&self) -> gdk_pixbuf::Pixbuf {
        let pixmap = ImageBuilder::default()
            .shape(Shape::RoundedSquare)
            .background_color([255, 255, 255, 255]) // transparency
            .fit_width(PIXBUF_WIDTH)
            .to_pixmap(&self.code);
        let width = pixmap.width();
        let height = pixmap.height();
        let pixels = pixmap.take();
        let bytes: glib::Bytes = glib::Bytes::from(&pixels);

        gdk_pixbuf::Pixbuf::from_bytes(&bytes,
                                       gdk_pixbuf::Colorspace::Rgb,
                                       true,
                                       8,
                                       width as i32, height as i32,
                                       width as i32 * 4)
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
