# 📥 Install **`reqmd`**

## 🪟 Windows Installer

Download Windows Installer from latest release:

🔗 **[v0.1.1/reqmd_cli-x86_64-pc-windows-msvc.msi](https://github.com/benfalk/req_md/releases/download/v0.1.1/reqmd_cli-x86_64-pc-windows-msvc.msi)**

## 🐚 Powershell One Liner

Install pre-built binary for Windows with PowerShell one liner:

<!-- markdownlint-disable MD013 -->
```bash
powershell -ExecutionPolicy Bypass -c "irm https://github.com/benfalk/req_md/releases/download/v0.1.1/reqmd_cli-installer.ps1 | iex"
```

## 🖥️ Linux/MacOS One Liner

Install pre-built binaries for Linux and MacOS with shell one liner:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/benfalk/req_md/releases/download/v0.1.1/reqmd_cli-installer.sh | sh
```
<!-- markdownlint-enable MD013 -->

## 🦀 With **`cargo install`**

Downloads latest source code from crates.io and compiles it locally

```bash
cargo install reqmd_cli
```

## 🦀 With **`cargo binstall`**

Install pre-built binaries for popular platforms

```bash
cargo binstall reqmd_cli
```

## 🦀 From Source

To install from source you will need the rust compiler tool-chain.  If you
don't have it installed, you can get it from [rustup.rs].  You will also
need [git] to fetch the source code.  Probably the biggest benefit from
installing from source is you can get the latest features and bug fixes
without waiting for a release.

### 1. **`git clone`**

```bash
git clone https://github.com/benfalk/req_md.git
```

### 2. **`cargo install`**

```bash
cargo install --path=req_md/crates/reqmd_cli
```

## 🌐 Pre-Built Binaries

All possible binaries for popular platforms are available for download
along with their respective checksums.  Source for most of the solutions
that are above.

🔗 **[benfalk/req_md/releases](https://github.com/benfalk/req_md/releases)**

[rustup.rs]: https://rustup.rs/
[git]: https://git-scm.com/
