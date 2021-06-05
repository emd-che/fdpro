use std::fs::File;
use std::io;
use std::path::Path;
use std::ffi::OsStr;

 enum ConfigFileType {
    TOML,
    JSON,
    YAML,
    INI,
    Other,
}
 enum MarkdownLanguage {
    HTML,
    XML,
    XAML,
    MD,
}
 enum FileType {
    SourceFile,
    RunnableScript,
    Image,
    Markdown(MarkdownLanguage),
    ExecutableBinary,
    ConfigFile(ConfigFileType),
    Other,
}

pub enum Files {
    Text(TextFile),
    Bin(BinFile)
}

fn get_file_type(ext: &str) -> FileType {
    match ext {
        "rs" | "fs" | "cpp" | "c" => FileType::SourceFile,
        "py" | "rb" | "js" => FileType::RunnableScript,
        _ => FileType::Other,
    }
}

trait ProjectFile {
    fn get_name(&self) -> String;
    fn get_path(&self) -> String;

    fn get_extention(&self) -> Option<String> {
        Some("rs".to_string())
        //TODO: Implement it
    }
    fn get_type(&self) -> Option<FileType> {
        match &self.get_extention() {
            Some(ext) => Some(get_file_type(&ext)),
            None => None,
        }
    }
}

 struct TextFile {
    pub file_name: String,
    pub file_path: String,
}
 struct BinFile {
    pub file_name: String,
    pub file_path: String,
}

impl ProjectFile for TextFile {
    fn get_name(&self) -> String {
        self.file_name
    }
    fn get_path(&self) -> String {
        self.file_path
    }
}
impl ProjectFile for BinFile {
    fn get_name(&self) -> String {
        self.file_name
    }
    fn get_path(&self) -> String {
        self.file_path
    }
}

impl TextFile {
    fn new(file_name: String, file_path: String) -> Self {
        TextFile {
            file_name,
            file_path
        }
    }
    fn get_loc(&self) -> Result<Option<usize>, io::Error> {
        unimplemented!();
    }
}
impl BinFile {
fn new(file_name: String, file_path: String) -> Self {
        BinFile {
            file_name,
            file_path
        }
    }
}

fn get_size(file: &impl ProjectFile) -> Result<Option<usize>, io::Error> {
    let path = Path::new(&file.get_path());
    let file_name = file.get_name();
    let ext = file.get_extention().unwrap();
    let full_path = path.join(format!("{}.{}", file_name, ext));
    let file = File::open(&full_path)?;
    match file.metadata() {
        Ok(metadata) => Ok(Some(metadata.len() as usize)),
        Err(e) => Err(e),
    }
    //TODO: make a new Error type and switch Option to Result
    //fn get_line_count(&self) -> Option<usize>{
    //}
}
pub fn new_project_file(filename: &str, filepath: &str) -> Files {
        let path = Path::new(filename);
        let ext: &str = path.extension().and_then(OsStr::to_str).unwrap(); //TODO: add proper error handling
        match get_file_type(ext) {
           FileType::SourceFile | FileType::RunnableScript | FileType::ConfigFile(_) | FileType::Markdown(_) 
               => Files::Text(TextFile::new(filename.to_string(), filepath.to_string())),
           FileType::ExecutableBinary | FileType::Image | FileType::Other 
               => Files::Bin(BinFile::new(filename.to_string(), filepath.to_string()))
        } 
}
//pub fn new_project_file(filename: &str, filepath: &str) -> Box<dyn ProjectFile> {
  //      let path = Path::new(filename);
 //       let ext: &str = path.extension().and_then(OsStr::to_str).unwrap(); //TODO: add proper error handling
  //      match get_file_type(ext) {
 //          FileType::SourceFile | FileType::RunnableScript | FileType::ConfigFile(_) | FileType::Markdown(_) 
   //            => Box::new(TextFile::new(filename.to_string(), filepath.to_string())),
//           FileType::ExecutableBinary | FileType::Image | FileType::Other 
    //           => Box::new(BinFile::new(filename.to_string(), filepath.to_string()))
   //     } 
//}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::fs::File;
    #[test]
    fn test_get_size() {
        let dir = env::current_dir().unwrap();
        let path = dir.join("src").join("main.rs");
        let path_display = path.display();
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => panic!("enable to open: {}: {}", path_display, why.to_string()),
        };
         match new_project_file("main", &dir.join("src").display().to_string()) {

        Files::Text(v) => {
            assert_eq!(
            get_size(&v).unwrap(),
            Some(file.metadata().unwrap().len() as usize)
        );
        assert_eq!(get_size(&v).unwrap().unwrap() != 0, true);
        } 
        Files::Bin(v) => {
 assert_eq!(
            get_size(&v).unwrap(),
            Some(file.metadata().unwrap().len() as usize)
        );
        assert_eq!(get_size(&v).unwrap().unwrap() != 0, true);
        }
        
         
    }
}}
