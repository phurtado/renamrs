#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn pad_int(n: u32) -> String {
    let n = n.to_string();
    std::iter::repeat("0").take(5 - n.len()).collect::<String>() + &n
}

pub fn rename_from_folder_name(path: &str, start: u32) -> Result<(), Box<dyn Error>> {
    let mut hmap: HashMap<String, String> = HashMap::new();
    let mut idx = start;
    let dirpath = Path::new(&path);
    let files = fs::read_dir(&dirpath)?;
    let dirname = match dirpath.file_name() {
        Some(p) => p.to_str().unwrap(),
        None => Err("Could not get actual directory name")?
    };
    
    for file in files {
        let fpath = &file?.path();
        let ftype = fpath.extension().and_then(OsStr::to_str).unwrap_or_else(|| "");
        let idx_str = &pad_int(idx)[..];
        let oldname = String::from(fpath.file_stem().unwrap().to_str().unwrap());

        let run_copy = |idx: &str| -> Result<(), Box<dyn Error>> {
            let newname = if ftype.is_empty() {
                format!("{}{}", dirname, idx)
            } else {
                format!("{}{}.{}", dirname, idx, ftype)
            };
            fs::copy(fpath, dirpath.join(newname))?;
            Ok(())
        };
        
        if let Some(idx_str) = hmap.get(&oldname) {
            run_copy(idx_str)?;
        }
        else {
            run_copy(idx_str)?;            
            hmap.insert(oldname, String::from(idx_str));
            idx += 1;
        }
    }
    Ok(())
}


