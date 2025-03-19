pub struct FileOrganizer;

impl FileOrganizer {
    pub fn create_folder(&self, folder_name: &str) -> std::io::Result<()> {
        std::fs::create_dir_all(folder_name)
    }

    pub fn move_file(&self, source: &str, destination: &str) -> std::io::Result<()> {
        std::fs::rename(source, destination)
    }

    pub fn list_files(&self, folder_name: &str) -> std::io::Result<Vec<String>> {
        let entries = std::fs::read_dir(folder_name)?
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().into_string().unwrap())
            .collect();
        Ok(entries)
    }
}