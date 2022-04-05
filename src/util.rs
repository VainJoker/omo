use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

//获取当前路径的子文件夹所有的内容
pub fn get_content(path: PathBuf) -> Vec<OsString> {
    let walker = WalkDir::new(&path)
        .max_depth(1)
        .min_depth(1)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    let mut contents: Vec<OsString> = Vec::new();
    if Path::is_dir(&path) {
        for entry in walker {
            match entry {
                Ok(entry) => {
                    let content = entry;
                    contents.push(content.file_name().to_os_string());
                }
                Err(_) => {}
            };
        }
    }
    contents
}

pub fn get_child_content(path: PathBuf) -> Vec<OsString> {
    let walker = WalkDir::new(&path)
        .max_depth(2)
        .min_depth(1)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    let mut contents: Vec<OsString> = Vec::new();
    if Path::is_dir(&path) {
        for entry in walker {
            match entry {
                Ok(entry) => {
                    let content = entry;
                    contents.push(content.file_name().to_os_string());
                }
                Err(_) => {}
            };
        }
    }
    contents
}

pub fn get_item_len(pb: PathBuf) -> f32 {
    if Path::is_file(&pb) {
        let metadata = pb.metadata().unwrap();
        return metadata.len() as f32;
    } else if Path::is_dir(&pb) {
        let walker = WalkDir::new(&pb).min_depth(1).into_iter();
        let mut len: f32 = 0.0;
        for entry in walker {
            match entry {
                Ok(entry) => {
                    let content = entry;
                    len += content.path().to_path_buf().metadata().unwrap().len() as f32;
                }
                Err(_) => {}
            };
        }
        return len as f32;
    } else {
        0.0
    }
}

pub fn show_item_size(size: f32) -> String {
    if size < 1024.0 {
        return format!("{:.2}B", size);
    } else if size < 1024.0 * 1024.0 {
        return format!("{:.2}KB", size / 1024.0);
    } else if size < 1024.0 * 1024.0 * 1024.0 {
        return format!("{:.2}MB", size / 1024.0 / 1024.0);
    } else if size < 1024.0 * 1024.0 * 1024.0 * 1024.0 {
        return format!("{:.2}GB", size / 1024.0 / 1024.0 / 1024.0);
    } else {
        return format!("{:.2}TB", size / 1024.0 / 1024.0 / 1024.0 / 1024.0);
    }
}
