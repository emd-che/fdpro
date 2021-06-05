use super::project_file::{new_project_file, Files};
use super::traits::Size;
use std::io;

pub struct ProjectDir {
    pub dir_name: String,
    pub path: String,
    files: Vec<Files>,
}

impl ProjectDir {
    pub fn get_files(&self) -> Option<Vec<Files>> {
        let files: Vec<Files> = self.files.into_iter().map(|f| {
            match f {
                Files::Text(vtext) => vtext,
                Files::Bin(vbin) => vbin,
            }
        }).collect();
        files
        //TODO: implement it
        //Remember ls command
    }
}

impl Size for ProjectDir {
    fn get_size(&self) -> Result<Option<usize>, io::Error> {
        match &self.get_files() {
            Some(files) if files.len() > 1 => {
                Ok(Some(files.iter().fold(
                    0 as usize,
                    |acc, f| match f.get_size() {
                        Ok(size) => acc + size.unwrap(),
                        Err(e) => std::panic::panic_any(e),
                    },
                )))
            }
            Some(files) if files.len() == 1 => Ok(Some(files[0].get_size()?.unwrap())),
            // Some(files) if files.len() <= 0 => Ok(Some(0)),
            None => Ok(None),
            _ => Ok(None),
        }
    }
}
