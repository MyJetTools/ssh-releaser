pub struct PopulateVariablesProcessing {
    raw: bool,
    url_encoded: bool,
}

impl PopulateVariablesProcessing {
    pub fn new(src: &str, placeholder: &str) -> Self {
        let mut raw = false;
        let mut url_encoded = false;

        for itm in src.split(":") {
            match itm {
                "raw" => {
                    raw = true;
                }
                "url_encoded" => {
                    url_encoded = true;
                }
                _ => {
                    panic!(
                        "Unknown post processing suffix {} for placeholder {}",
                        itm, placeholder
                    );
                }
            }
        }
        Self { raw, url_encoded }
    }

    pub fn empty() -> Self {
        Self {
            raw: false,
            url_encoded: false,
        }
    }

    pub fn has_raw(&self) -> bool {
        self.raw
    }

    pub fn has_url_encoded(&self) -> bool {
        self.url_encoded
    }
}
