// Copy the src/www folder into the OUT_DIR folder so we can easily access it.

use std::env;
use std::path::Path;

extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
    let profile = env::var_os("PROFILE").unwrap();
    let profile = profile.into_string().unwrap();
    let current_dir = env::current_dir().unwrap();
    let base_dir = Path::new(&current_dir);
    let source_dir = base_dir.join("src/www");
    let target_dir = base_dir.join("target").join(&profile);

    let mut options = CopyOptions::new();
    options.overwrite = true;

    copy(source_dir, target_dir, &options).unwrap();
}
