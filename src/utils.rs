use dir_spec::Dir;
use std::fs;
use std::path::PathBuf;

pub const PROJECT_LOCAL_FOLDER: &str = "magic_finder";
pub const SQLITE_FILENAME: &str = "magic_filder_db.sqlite3";

pub fn get_local_data_folder() -> PathBuf {
    let data_folder = Dir::data_home();
    match data_folder {
        None => {
            panic!("Can't find a data folder - really don't know what the problem is sorry");
        }
        Some(mut f) => {
            f.push(PROJECT_LOCAL_FOLDER);
            f
        }
    }
}

fn get_local_data_sqlite_file() -> PathBuf {
    let mut folder = get_local_data_folder();
    folder.push(SQLITE_FILENAME);
    folder
}

// NOTE: this should be idempotent - creating a dir always is... right?
pub fn create_local_data_folder() {
    let f = get_local_data_folder();
    let ret = fs::create_dir(&f);
    match ret {
        Ok(_) => (),
        Err(e) => {
            if e.raw_os_error().unwrap() == 17 {
                // 17 = this is folder already exists - which is fine for us
                // TODO probably should use e.kind() for better readability
                return;
            }
            panic!(
                "Couldn't create folder within your cache folder: {}. Error is {}",
                f.display(),
                e
            );
        }
    }
}
