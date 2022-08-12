use std::path::PathBuf;

wit_bindgen_rust::export!("./assets/manifest.wit");
wit_bindgen_rust::export!("./assets/filesystem.wit");

struct Manifest();

impl manifest::Manifest for Manifest {
    fn name() -> String {
        "Blockstorage".into()
    }
    fn description() -> String {
        "Blockstorage Plugin provides a common directory to store and retrive all ZeroNetX File, this will eliminate any duplicate files accross ZeroNet Sites".into()
    }
    fn version() -> i64 {
        0
    }
    fn permissions() -> Vec<String> {
        vec![]
    }
}

struct Filesystem();

impl filesystem::Filesystem for Filesystem {
    fn get_storage_path(data_path: String) -> String {
        let data_path = PathBuf::from(data_path);
        data_path.join("blockstorage").display().to_string()
    }

    fn get_file_path(data_path: String, block_id: String) -> String {
        let data_path = PathBuf::from(data_path);
        data_path
            .join("blockstorage")
            .join(block_id)
            .display()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{filesystem::Filesystem, Filesystem as FilesystemTest};
    use std::path::Path;

    #[test]
    fn test_get_block_storage_path() {
        let block_storage_path: String = FilesystemTest::get_storage_path("data/path".into());
        assert_eq!(
            Path::new(&block_storage_path),
            Path::new("/tmp/data/blockstorage")
        );
    }

    #[test]
    fn test_get_block_file_path() {
        let block_file_path = FilesystemTest::get_file_path("/tmp/data".into(), "block_id".into());
        assert_eq!(
            Path::new(&block_file_path),
            Path::new("/tmp/data/blockstorage/block_id")
        );
    }
}
