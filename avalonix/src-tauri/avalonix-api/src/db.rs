use media::track;
use sled;

fn open_db(folder_name: String) {
    let db_dest = dirs::home_dir() + "/.avalonix/db/" + folder_name;
    let db = sled::open(db_dest);

    match db {
        Ok(db) => {
            info("DB (" + db_dest + ") opened");
        }

        Err(e) => {
            error("DB open error: " + &e);
        }
    }
}

fn save_track(db: Db, track: Track) {
    db.insert(track.metadata.title, track);
}

fn delete_track(db: Db, track: Track) {
    db.remove(track.metadata.title);
}

fn delete_track_by_name(db: Db, track_name: String) {
    db.remove(track_name);
}

