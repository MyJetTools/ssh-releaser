use crate::file_path::FilePathRef;

#[derive(Clone)]
pub struct FileName(String);

impl FileName {
    pub fn new(src: String) -> Self {
        Self(src)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn get_file_path<'s>(&'s self) -> FilePathRef<'s> {
        let src = self.0.as_bytes();

        let mut index = src.len();

        while index > 0 {
            if let Some(b'/') = src.get(index - 1) {
                return FilePathRef::new(std::str::from_utf8(&src[0..index]).unwrap());
            }
            index -= 1;
        }

        FilePathRef::new(self.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::file_name::FileName;

    #[test]
    fn test_get_file_path() {
        let src = "/test/test2/my_file.yaml".to_string();

        let file_name = FileName::new(src);

        assert_eq!("/test/test2/", file_name.get_file_path().as_str());
    }

    #[test]
    fn test_get_file_path_root() {
        let src = "/my_file.yaml".to_string();

        let file_name = FileName::new(src);

        assert_eq!("/", file_name.get_file_path().as_str());
    }
}
