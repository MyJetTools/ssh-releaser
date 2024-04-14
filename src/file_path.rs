#[derive(Clone, Debug)]
pub struct FilePath(String);

impl FilePath {
    pub fn new(src: String) -> Self {
        Self(src)
    }

    pub fn as_ref(&self) -> FilePathRef {
        FilePathRef::new(self.0.as_str())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct FilePathRef<'s>(&'s str);

impl<'s> FilePathRef<'s> {
    pub fn new(src: &'s str) -> Self {
        Self(src)
    }

    pub fn to_owned(&self) -> FilePath {
        FilePath::new(self.0.to_string())
    }

    pub fn as_str(&'s self) -> &'s str {
        self.0
    }
}
