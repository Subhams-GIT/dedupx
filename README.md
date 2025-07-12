<p align="center">
    <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" align="center" width="30%">
</p>
<p align="center"><h1 align="center">DEDUPX</h1></p>
<p align="center">
	<em><code>❯ File Deduplicator using rust</code></em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/Subhams-GIT/dedupx?style=flat-square&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/Subhams-GIT/dedupx?style=flat-square&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/Subhams-GIT/dedupx?style=flat-square&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/Subhams-GIT/dedupx?style=flat-square&color=0080ff" alt="repo-language-count">
</p>
<p align="center">Built with the tools and technologies:</p>
<p align="center">
	<img src="https://img.shields.io/badge/Rust-000000.svg?style=flat-square&logo=Rust&logoColor=white" alt="Rust">
	<img src="https://img.shields.io/badge/JavaScript-F7DF1E.svg?style=flat-square&logo=JavaScript&logoColor=black" alt="JavaScript">
</p>
<br>

##  Table of Contents

- [ Overview](#-overview)
- [ Features](#-features)
- [ Project Structure](#-project-structure)
  - [ Project Index](#-project-index)
- [ Getting Started](#-getting-started)
  - [ Prerequisites](#-prerequisites)
  - [ Installation](#-installation)
  - [ Usage](#-usage)
  - [ Testing](#-testing)
- [ Project Roadmap](#-project-roadmap)
- [ Contributing](#-contributing)
- [ License](#-license)
- [ Acknowledgments](#-acknowledgments)

---

##  Overview

<code>❯ Get your duplicate files occupy no more space </code>

---

##  Features

<code>❯ Detects duplicate files in folder with custom speed </code>

---

##  Project Structure

```sh
└── dedupx/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── doc.md
    ├── quarantine
    │   ├── file1.js
    │   ├── file2.js
    │   ├── main
    │   ├── quarantine_report.json
    │   ├── t1.txt
    │   ├── t2.txt
    │   └── t3.txt
    ├── src
    │   ├── delete.rs
    │   ├── find.rs
    │   ├── getconfig.rs
    │   ├── getentries.rs
    │   ├── hash.rs
    │   ├── lib.rs
    │   ├── main.rs
    │   ├── read.rs
    │   └── report.rs
    └── tests
        ├── delete_test.rs
        ├── find_test.rs
        ├── getconfig-test.rs
        └── getentries_test.rs
```


###  Project Index
<details open>
	<summary><b><code>DEDUPX/</code></b></summary>
	<details> <!-- __root__ Submodule -->
		<summary><b>__root__</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/Cargo.toml'>Cargo.toml</a></b></td>
				<td><code>❯ REPLACE-ME</code></td>
			</tr>
			</table>
		</blockquote>
	</details>
	<details> <!-- quarantine Submodule -->
		<summary><b>quarantine</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/quarantine/main'>main</a></b></td>
				<td><code>❯ main entry of project </code></td>
      </tr>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/quarantine/quarantine_report.json'>quarantine_report.json</a></b></td>
				<td><code>❯ quarantine report </code></td>
      </tr>
			</table>
		</blockquote>
	</details>
	<details> <!-- src Submodule -->
		<summary><b>src</b></summary>
		<blockquote>
			<table>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/src/lib.rs'>lib.rs</a></b></td>
				<td><code>❯ declares all modules</code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/src/read.rs'>read.rs</a></b></td>
				<td><code>❯ read the skips list </code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/src/getconfig.rs'>getconfig.rs</a></b></td>
				<td><code>❯ gets the config of user </code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/src/hash.rs'>hash.rs</a></b></td>
				<td><code>❯ initialises and hashes the data </code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/src/delete.rs'>delete.rs</a></b></td>
				<td><code>❯ deletes and quarantines the files</code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/src/getentries.rs'>getentries.rs</a></b></td>
				<td><code>❯ gets the valid entries </code></td>
			</tr>
			<tr>
				<td><b><a href='https://github.com/Subhams-GIT/dedupx/blob/master/src/find.rs'>find.rs</a></b></td>
				<td><code>❯ find the duplicates </code></td>
			</tr>
			</table>
		</blockquote>
	</details>
</details>

---
##  Getting Started

###  Prerequisites

Before getting started with dedupx, ensure your runtime environment meets the following requirements:

- **Programming Language:** Rust
- **Package Manager:** Cargo


###  Installation

Install dedupx using one of the following methods:

**Build from source:**

1. Clone the dedupx repository:
```sh
❯ git clone https://github.com/Subhams-GIT/dedupx
```

2. Navigate to the project directory:
```sh
❯ cd dedupx
```

3. Install the project dependencies:


**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo build
```




###  Usage
Run dedupx using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo run
```


###  Testing
Run the test suite using the following command:
**Using `cargo`** &nbsp; [<img align="center" src="https://img.shields.io/badge/Rust-000000.svg?style={badge_style}&logo=rust&logoColor=white" />](https://www.rust-lang.org/)

```sh
❯ cargo test
```


---
##  Project Roadmap

- [X] **`Task 1`**: <strike>Implement feature one.</strike>
- [ ] **`Task 2`**: Implement feature two.
- [ ] **`Task 3`**: Implement feature three.

---

##  Contributing

- **💬 [Join the Discussions](https://github.com/Subhams-GIT/dedupx/discussions)**: Share your insights, provide feedback, or ask questions.
- **🐛 [Report Issues](https://github.com/Subhams-GIT/dedupx/issues)**: Submit bugs found or log feature requests for the `dedupx` project.
- **💡 [Submit Pull Requests](https://github.com/Subhams-GIT/dedupx/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/Subhams-GIT/dedupx
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to github**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="left">
   <a href="https://github.com{/Subhams-GIT/dedupx/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=Subhams-GIT/dedupx">
   </a>
</p>
</details>

---

##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

---
