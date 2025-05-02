# OneFile

*Flatten an entire source tree into **one** UTF‑8 text file – perfect for pasting into ChatGPT for code review, quick archiving, or sharing a snapshot of your code.*

---

## ✨ What it does
```text
project-root/
└─ src/
   ├─ main.rs
   └─ utils/
      └─ helper.rs
```
⬇︎
```text
flattened_code.txt
┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄
----- src/main.rs -----
<file contents>

----- src/utils/helper.rs -----
<file contents>
```

* Streams every file under a folder **recursively** into one well‑segmented text file.
* **Skips binary files** by default (enable `-b` to include them).
* Ships as a single native binary – **no Python**, no runtime, no venv.

---

## 📦 Download the latest binaries

| OS | File |
|----|------|
| Windows (.exe) | [onefile‑windows.exe](https://github.com/SebastianArthurKull/OneFile/releases/latest/download/onefile-windows.exe) |
| macOS | [onefile‑macos](https://github.com/SebastianArthurKull/OneFile/releases/latest/download/onefile-macos) |
| Linux | [onefile‑linux](https://github.com/SebastianArthurKull/OneFile/releases/latest/download/onefile-linux) |

## 🛠️ Build from source
> **Prerequisite:** [Rust toolchain](https://rustup.rs) ≥ 1.70.

```bash
# in the repo root (where Cargo.toml lives)
cargo build --release           # → target/release/flatten-folder(.exe)
```

## 🚀 Usage
```bash
flatten-folder <FOLDER> [options]

Options:
  -o, --output <FILE>     Output path (default: ./flattened_code.txt)
  -b, --binary            Include binary files (raw bytes)
  -v, --verbose           Print every processed path
  -h, --help              Show help
  -V, --version           Show version
```

### Path basics (Windows vs. Unix)
* **Relative paths** (`.` or `..`) work on all platforms.
* On **Windows**, a path that begins with `/` or `\` is treated as *absolute from the drive root* (e.g. `/apps` → `C:\apps`). If that directory doesn’t exist you’ll see *“is not a directory”*.
* Use one of:
  * `flatten-folder .`  → current directory
  * `flatten-folder ..\apps\SnapQuest\src`  → relative path up one level
  * `flatten-folder C:\full\path\to\src`  → full absolute path

### Examples
| Goal | Command (PowerShell/Bash) |
|------|---------------------------|
| Flatten current repo | `./tools/flatten-folder .` |
| Verbose progress & custom name | `flatten-folder src -o snapshot.txt -v` |
| Include binary assets (fonts, icons, …) | `flatten-folder assets -b` |
| Flatten sibling folder (Windows) | `flatten-folder ..\apps\MyApp\src` |
| Flatten with an absolute path (Unix) | `flatten-folder /home/user/project/src` |

---

## 📝 How it works
* [`walkdir`](https://crates.io/crates/walkdir) streams directory entries efficiently.
* Detects binaries by attempting to decode the first 512 bytes as UTF‑8.
* Writes through a buffered writer → **O(1) memory** even for very large trees.

---

## 💡 Extending
Need file‑type filters, `.gitignore` support, compression, or hash manifests? Open an issue or start a discussion.

---

## 📄 License
MIT – do what you like, just keep the copyright notice.

