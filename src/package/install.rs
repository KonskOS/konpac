extern crate flate2;
extern crate tar;
extern crate serde_json;
extern crate rusqlite;

use rusqlite::{params, Connection, Result};
use serde_json::{Result,Value};
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archinve;

struct Package {
    id: i32,
    name: String,
    author: String,
    version: String,
    root: String,
    user: String

}


mod database {
    fn add_package(package: Package){
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "INSERT INTO packages (name, author, version, root, user) VALUES (?1,?2,?3,?4,?5)",
            (&package.name, &package.author, &package.version, &package.root, &package.user)
        )
    }

}


fn get_name(path: String) {
    let path_split: [&str; path.len()] = path.split("/");
    let archive: &str = path_split[path.len()-1];
    let archive_split: [&str; archive.len()] = archive.split(".");
    let name: &str = archive_split[0];
    name
}

fn get_info(about_path: String) -> Value{
    let about = fs::read_to_string(about_path).expect("Error in read about");
    let value: Value = serde_json::from_str(about)?;

    value
}

pub fn install(path: &str,root: &str, is_global: Bool,user: &str) -> Result<T> {
    let kpkg = File::open(path)?;
    let tar = GzDecoder::new(kpkg);
    let mut archinve = Archinve::new(tar);
    let temp_dir: &str = String::new("{}/tmp/{}",root,get_name(path));
    archinve.unpack(temp_dir);
    let about_file: &str = String::new("{}/.package/about.json",temp_dir);
    let about_info: Value = get_info(about_file);
    let cur_package: Package = Package{
        name: &str = about_info["name"],
        author: &str = about_info["author"],
        version: &str = about_info["version"],
        root: &str = root,
        user: &str = user
    };


}