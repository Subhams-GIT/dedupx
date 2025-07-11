# 🔍 DedupX — Fast Duplicate File Finder in Rust

**DedupX** is a high-performance, parallel duplicate file finder written in Rust. It scans a directory for files with identical content by hashing them, helping you reclaim disk space and eliminate clutter.

---

## 🚀 Features

- ✅ **Blazing-fast** duplicate detection using `rayon` for parallel hashing
- ⚡ **Configurable scan speed** (`Slow`, `Medium`, `Fast`)
- 📁 **Target any folder** with the `--folder` CLI option
- 🛠️ Supports multiple hash algorithms (e.g., `sha256`, `md5`, `blake3`)
- 🧠 Skips directories or files via pattern matching
- 🔒 Built with Rust's safety and performance in mind

---

## ⚙️ Usage

### 🔧 Command-line Options

```bash
dedupx --folder <FOLDER_PATH> --speed <SPEED>
