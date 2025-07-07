use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

pub struct Quarantine {
    dir: PathBuf,
    reporter: ReportBuilder,
}

#[derive(Serialize, Deserialize)]
pub struct ReportData {
    timestamp: String,
    quarantined_files: Vec<QuarantinedFile>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QuarantinedFile {
    og_path: PathBuf,
    q_path: PathBuf,
    size: u64,
    timestamp: String,
}

pub struct ReportBuilder {
    data: ReportData,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self {
            data: ReportData {
                timestamp: Local::now().to_rfc3339(),
                quarantined_files: Vec::new(),
            },
        }
    }

    pub fn add_quarantined_file(&mut self, og_path: PathBuf, q_path: PathBuf, size: u64) {
        let file = QuarantinedFile {
            og_path,
            q_path,
            size,
            timestamp: Local::now().to_rfc3339(),
        };
        self.data.quarantined_files.push(file);
    }

    pub fn generate_report(&self) -> ReportData {
        ReportData {
            timestamp: Local::now().to_rfc3339(),
            quarantined_files: self.data.quarantined_files.clone(),
        }
    }

    pub fn save_json_report(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let report = self.generate_report();
        let path = path.as_ref();
        
        // Try to read existing data if file exists
        let mut combined_data = if path.exists() {
            let contents = fs::read_to_string(path)?;
            serde_json::from_str(&contents).unwrap_or_else(|_| ReportData {
                timestamp: Local::now().to_rfc3339(),
                quarantined_files: Vec::new(),
            })
        } else {
            ReportData {
                timestamp: Local::now().to_rfc3339(),
                quarantined_files: Vec::new(),
            }
        };

        // Merge with new data
        combined_data.quarantined_files.extend(report.quarantined_files);
        combined_data.timestamp = Local::now().to_rfc3339();

        let json = serde_json::to_string_pretty(&combined_data)?;
        fs::write(path, json)
    }
}

impl Quarantine {
    pub fn qdir() -> io::Result<Self> {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let quarantine_dir = project_root.join("quarantine");
        fs::create_dir_all(&quarantine_dir)?;

        Ok(Self {
            dir: quarantine_dir,
            reporter: ReportBuilder::new(),
        })
    }

    pub fn quarantine_duplicates(
        &mut self,
        groups: Vec<Vec<PathBuf>>,
        option: &str,
    ) -> io::Result<Vec<PathBuf>> {
        let mut quarantined = Vec::new();

        for group in groups {
            if group.len() > 1 {
                for file in group.into_iter().skip(1) {
                    println!("Processing file: {}", file.display());

                    if !file.exists() {
                        println!("Warning: File {} does not exist, skipping", file.display());
                        continue;
                    }

                    match self.q_file(&file, option) {
                        Ok(path) => {
                            println!("Successfully processed: {}", file.display());
                            quarantined.push(path);
                        }
                        Err(e) => {
                            println!("Error processing {}: {}", file.display(), e);
                        }
                    }
                }
            }
        }

        Ok(quarantined)
    }

    pub fn q_file(&mut self, path: impl AsRef<Path>, option: &str) -> io::Result<PathBuf> {
        let file_path = path.as_ref();
        let size = fs::metadata(file_path)?;
        let file_name = file_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path has no filename"))?;

        let q_path = self.dir.join(file_name);
        let result_path=if option=="Quarantine"{
            fs::rename(file_path, &q_path)?;
                q_path.clone()
        } else{
            fs::remove_file(file_path)?;
                PathBuf::from("DELETED")
        };
        

        self.reporter.add_quarantined_file(
            file_path.to_path_buf(),
            result_path.clone(),
            size.len(),
        );

        let report_path = self.dir.join("quarantine_report.json");
        self.reporter.save_json_report(report_path)?;

        Ok(result_path)
    }
}