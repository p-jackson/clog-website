use std::fs::{self, File};
use std::path::Path;
use std::io::Read;
use std::error::Error;
use clog::Clog;

pub fn generate_changelog(repository: &str, repo_url: &str) -> Result<String, Box<Error>> {
    let mut clog = try!(Clog::with_dir(repository).map_err(|e| {
        fs::remove_dir_all(repository).ok();
        e
    }));
    let changelog_file_name = format!("changelog_{}.md", repository);
    clog.repository(repo_url);
    try!(clog.write_changelog_to(Path::new(repository).join(&changelog_file_name)));

    let mut contents = String::new();

    try!(File::open(&Path::new(repository).join(&changelog_file_name))
        .map(|mut f| f.read_to_string(&mut contents).ok()));

    fs::remove_dir_all(repository).ok();

    Ok(contents)
}
