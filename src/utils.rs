mod path_utils {
    use std::path::{Path, PathBuf};

    pub fn get_audio_file_path(file_name: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push("recordings");
        path.push(file_name);
        path
    }

    pub fn create_directory_if_not_exists(dir: &str) -> std::io::Result<()> {
        if !Path::new(dir).exists() {
            std::fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}

mod error_handling {
    use std::fmt;

    #[derive(Debug)]
    pub enum AppError {
        IoError(std::io::Error),
        TranscriptionError(String),
        SummarizationError(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl From<std::io::Error> for AppError {
        fn from(error: std::io::Error) -> Self {
            AppError::IoError(error)
        }
    }
}