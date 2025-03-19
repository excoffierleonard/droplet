pub mod services {
    use actix_files::Files;
    use std::path::PathBuf;

    // This function creates a new scope that serves files from a directory.
    pub fn serve_dir(dir: &PathBuf) -> Files {
        Files::new("/", dir).show_files_listing()
    }
}
