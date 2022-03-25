use std::{collections::BTreeMap, env::current_dir, ffi::OsString, path::PathBuf};

use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Node {
    pub current_path: PathBuf,
    pub tc: BTreeMap<OsString, Vec<OsString>>,
    pub tp: BTreeMap<OsString, Vec<OsString>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::new(),
            tc: BTreeMap::new(),
            tp: BTreeMap::new(),
        }
    }

    pub async fn default(&mut self) -> Self {
        self.set_current_path();
        self.set_tp().await;
        self.set_tc().await;
        Self {
            current_path: self.to_owned().current_path,
            tc: self.to_owned().tc,
            tp: self.to_owned().tp,
        }
    }

    //存入parent路径下的所有目录
    pub async fn set_tp(&mut self) {
        self.tp = BTreeMap::new();
        let mut parent: Vec<OsString> = Vec::new();
        match self.current_path.parent() {
            Some(i) => {
                for entry in WalkDir::new(i)
                    .max_depth(1)
                    .min_depth(1)
                    // .follow_links(true)
                    .sort_by_file_name()
                {
                    match entry {
                        Ok(entry) => parent.push(entry.file_name().to_os_string()),
                        Err(_) => {}
                    };
                }
                let c = OsString::from(
                    self.current_path
                        .file_name()
                        .clone()
                        .expect("get file_name error"),
                );
                self.tp.insert(c, parent);
            }
            None => {}
        }
    }

    //存入当前路径下所有子文件对应的孙子文件
    //此处，链接的文件会消失,错误处理需要更改
    //todo！
    pub async fn set_tc(&mut self) {
        self.tc = BTreeMap::new();
        let child: Vec<OsString> = Vec::new();
        for entry in WalkDir::new(self.current_path.clone())
            .max_depth(1)
            .min_depth(1)
            // .follow_links(true)
            .sort_by_file_name()
        {
            let mut child = child.clone();
            let mut path = self.clone().current_path;
            match &entry {
                Ok(entry) => {
                    path = entry.path().to_path_buf();
                }
                //应该问题在这
                Err(_) => {}
            };
            for child_entry in WalkDir::new(path)
                .max_depth(1)
                .min_depth(1)
                .follow_links(true)
                .sort_by_file_name()
            {
                match child_entry {
                    Ok(c) => child.push(c.file_name().to_os_string()),
                    //或者这
                    Err(_) => {}
                }
            }
            match &entry {
                Ok(entry) => {
                    let file_name = entry.file_name().to_os_string();
                    self.tc.insert(file_name.to_os_string(), child);
                }
                Err(_) => {}
            };
        }
    }

    fn set_current_path(&mut self) {
        let mut current_path = PathBuf::new();
        let walker = WalkDir::new(current_dir().expect("get current_dir error"))
            .max_depth(0)
            .follow_links(true)
            .sort_by_file_name()
            .into_iter();
        for entry in walker {
            match entry {
                Ok(entry) => current_path = entry.path().to_path_buf(),
                Err(_) => {}
            }
        }
        self.current_path = current_path;
    }
}
