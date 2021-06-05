use std::io;

pub trait Size {
    fn get_size(&self) -> Result<Option<usize>, io::Error>;
}
