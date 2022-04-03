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
