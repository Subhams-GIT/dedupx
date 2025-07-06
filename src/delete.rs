use std::io::{self};
use std::path::{Path, PathBuf};
use std::{fs};

pub struct Quarantine {
    dir: PathBuf,
}


impl Quarantine {
    pub fn qdir() -> io::Result<Self> {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let quarantine_dir = project_root.join("quarantine");
        
        fs::create_dir_all(&quarantine_dir)?;
        Ok(Self { dir: quarantine_dir })
    }

    pub fn quarantine_duplicates(&self, groups: Vec<Vec<PathBuf>>) -> io::Result<Vec<PathBuf>> {
        let mut quarantined = Vec::new();
        
        for (_idx, group) in groups.iter().enumerate() {
            
            if group.len() > 1 {
                for (file_idx, file) in group.iter().enumerate().skip(1) {
                    println!("  File {}: {}", file_idx, file.display());
                    
                    if !file.exists() {
                        println!("  WARNING: File does not exist, skipping");
                        continue;
                    }
                    
                    match self.q_file(file) {
                        Ok(path) => {
                            println!("  Successfully quarantined to: {}", path.display());
                            quarantined.push(path);
                        }
                        Err(e) => {
                            println!("  ERROR quarantining: {}", e);
                        }
                    }
                }
            }
        }
        
        Ok(quarantined)

    }

    pub fn q_file(&self, path: impl AsRef<Path>) -> io::Result<PathBuf> {
        let file_path = path.as_ref();
        
        let file_name = file_path.file_name()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::InvalidInput, 
                "Path has no filename"
            ))?;
        
        let q_path = self.dir.join(file_name);
        
        fs::rename(file_path, &q_path)?;
        
        Ok(q_path)
    }

}
