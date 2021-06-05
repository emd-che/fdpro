use super::project_dir::ProjectDir;

#[derive(Debug)]
pub enum Language {
    Rust,
    Cpp,
    Python,
    FSharp,
    CSharp,
    Go,
    JavaScript,
    Other(String),
}

//trait LineCount {
//    fn get_line_count(&self) -> Result<Option<usize>, io::Error>;
//}

pub struct ProjectInfo {
    pub title: String,
    pub path: String,
    pub dirs: Option<Vec<ProjectDir>>,
    pub number_of_files: Option<usize>,
    pub total_loc: Option<usize>,
    pub size: Option<usize>,
}

//TODO: Implement Size for ProjectInfo
