use std::collections::HashMap;

use crate::media::metadata::HashableMetadata;

pub struct Hash {
    pub primary_metadata: HashMap<String, HashableMetadata>,
}

impl Hash {
    pub fn from() -> Hash {
        // Need to implement loading from a hash file
        Hash {
            primary_metadata: HashMap::new(),
        }
    }

    pub fn add_track_to_hash(&mut self, path_to_track: &str, metadata: HashableMetadata) {
        self.primary_metadata
            .insert(path_to_track.to_string(), metadata);
    }
}

#[test]
fn test_hash_from() {}

#[test]
fn test_add_track_to_hash() {
    let mut hash = Hash::from();
    let track_path = "D:\\music\\metallica\\And justice for all\\08 - To Live is to Die.flac";
    let metadata = HashableMetadata::from(track_path);
    hash.add_track_to_hash(track_path, metadata);

    println!("{:#?}", hash.primary_metadata);
}
