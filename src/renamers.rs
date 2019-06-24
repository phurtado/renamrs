#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

fn pad_int(n: u32) -> String {
    let n = n.to_string();
    std::iter::repeat("0").take(5 - n.len()).collect::<String>() + &n
}

fn run_copy(dirpath: &Path, dirname_str: &str, 
            fpath: PathBuf, ftype: &str, idx_str: &str) -> Result<(), Box<dyn Error>> {
    let newname = if ftype.len() > 0 {
        format!("{}{}.{}", dirname_str, idx_str, ftype)
    } else {
        format!("{}{}", dirname_str, idx_str)
    };
    fs::copy(fpath, dirpath.join(newname))?;
    Ok(())
}

pub fn rename_from_folder_name(path: &str, start: u32) -> Result<(), Box<dyn Error>> {
    let mut hmap: HashMap<String, String> = HashMap::new();
    let dirpath = Path::new(&path);
    let files = fs::read_dir(&dirpath)?;
    let dirname = match dirpath.file_name() {
        Some(p) => p,
        None => Err("Could not get actual directory name")?
    };
    let dirname_str = dirname.to_str().unwrap();
    let mut idx = start;
    for file in files {
        let file = file?;
        let fpath = &file.path();
        let ftype = fpath.extension().and_then(OsStr::to_str).unwrap_or_else(|| "");
        let idx_str = &pad_int(idx)[..];
        let oldname = String::from(fpath.file_stem().unwrap().to_str().unwrap());
        
        if let Some(idx_str) = hmap.get(&oldname) {
            run_copy(dirpath, dirname_str, file.path(), ftype, idx_str)?;
        }
        else {
            run_copy(dirpath, dirname_str, file.path(), ftype, idx_str)?;
            
            hmap.insert(oldname, String::from(idx_str));
            idx += 1;
        }
    }
    Ok(())
}


