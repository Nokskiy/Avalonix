use super::metadata::Metadata;
use std::path::Path;

pub struct Track {
    pub metadata: Metadata,
    pub file_path: Path 
}
