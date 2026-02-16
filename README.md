# Xylo - Project maker

## Table of Contents
- [Installation](#Installation)
- [Usage](#Usage)
- [Configuration](#Configuration)
  - [Configuration File Structure](#Configuration-File-Structure)
  - [Default Template](#Default-Template)
  - [Example Configuration](#Example-Configuration)

---

## Installation

1. Clone the repository:
```
git clone https://github.com/aniko33/xylo.git
cd xylo
```

2. Install Xylo globally (ensure you have Rust installed):
```
cargo install --path .
```

3. Run Xylo:
```
xylo
```

---

## Usage

```
Usage: xylo [OPTIONS] <PATH>

Arguments:
  <PATH>  Project destination path

Options:
      --no-git             Skip Git initialization
  -f, --force              Overwrite existing files
  -t, --target <TARGET>    Set Clang compilation target
  -p, --profile <PROFILE>  Select a config profile from `xylo.toml`
  -h, --help               Print help
```

### To create a new project:
```
xylo <project-name>
```

### To create a new project based on an existing profile:
```
xylo <project-name> --profile <profile-name>
```

### To create a project without initializing Git:
```
xylo <project-name> --no-git
```

### To specify the Clang compilation target (e.g., `x86_64-linux-gnu`):
```
xylo <project-name> -t x86_64-linux-gnu
```

### To force overwrite existing files in the destination path:
```
xylo <project-name> -f
```

---

## Configuration

Configurations are stored in `~/.config/xylo/xylo.toml`.

### Configuration File Structure

The `xylo.toml` configuration file consists of multiple sections:

- `default_profile`: Specifies the default compilation profile.
- `profile`: An array of profiles, each containing:
  - `name`: Profile name.
  - `build`: Compilation settings.
    - `main_filename`: The name of the main file to compile.
    - `target` (optional): Compilation target, useful for cross-compilation.
    - `compiler`: Compiler settings.
      - `exec`: Compiler command.
      - `args`: Compiler arguments.
  - `structure`: Defines the project structure.
    - `directories`: Directories to be created automatically.
    - `files`: Files to be generated in the project.

### Default Template

```toml
default_profile = "template"

[[profile]]
name = "template"

[profile.build]
main_filename = "main"

[profile.build.compiler]
exec = "clang"
args = "-Iinclude -o target/main"

[profile.structure]
directories = ["src/", "target/", "include/"]
files = ["src/main.c"]
```

### Example Configuration

#### Windows Cross-platform Profile

```toml
default_profile = "windows_cross"

[[profile]]
name = "windows_cross"

[profile.build]
main_filename = "main"
target = "x86_64-pc-windows-gnu"

[profile.build.compiler]
exec = "clang"
args = "-Iinclude -o target/main"

[profile.structure]
directories = ["src/", "target/", "include/"]
files = ["src/main.c"]
```

#### Linux Profile

```toml
[[profile]]
name = "linux"

[profile.build]
main_filename = "main"

[profile.build.compiler]
exec = "clang"
args = "-Iinclude -o target/main"

[profile.structure]
directories = ["src/", "target/", "include/"]
files = ["src/main.c"]
```
