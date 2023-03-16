use fast_qr::qr::QRBuilder;

pub struct QRCode {
    code: fast_qr::qr::QRCode,
}

impl QRCode {
    pub fn from_str(contents: &str) -> Result<Self, String> {
        let code = QRBuilder::new(contents).build()
            .map_err(|_| format!("unable to build QR code"))?;

        Ok(QRCode { code })
    }

    pub fn to_ascii(self) -> String {
        self.code.to_str()
    }
}
