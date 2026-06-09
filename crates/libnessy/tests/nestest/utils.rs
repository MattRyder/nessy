use std::path::PathBuf;

pub fn get_asset_file_path(asset_file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .join("assets")
        .join(asset_file_name)
}
