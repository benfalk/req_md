<!-- markdownlint-disable-file MD024 -->
# 📑 Changelog

All notable changes to this project will be documented in this file. The format
is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## 🚧 [Unreleased]

## 🚀 [0.2.1] 2026-02-25

### ✨ Added

- Updated `reqmd_http::Headers` and `reqmd_http::QueryString` APIs:
  - `first_mut` for values to allow modification in place
  - `values_for_mut` to walk and update values for a given key
  - `delete_first` to remove the first key/value matching a key
  - `delete_all` to remove and collect all values for a key

### 🐛 Fixed

- Server from host plugin updates value with valid host name.  Prior to this
  fix it would leave the header as a formatted URL; which causes some servers
  to reject the request due to an invalid host header.

## 🚀 [0.2.0] 2026-02-23

🎨 Fit and finish for initial release with updated documentation.

### ✨ Added

- CI documentation job from master branch for GitHub Pages
- `dev-book` recipe to build and serve documentation locally
- `dev-setup` recipe and bootstrap script for local setup
- `reqmd` CLI documentation with examples and usage instructions
- `ServerFromHostname` processor plugin added

### ♻️ Changed

- Updated README with link to the online documentation

### 🐛 Fixed

- Allow `:` char in header values

## 🚀 [0.1.1] 2026-02-21

🚀 Initial release of ReqMD, a tool for parsing markdown files looking for code
blocks with a `http` language tag and formatted as HTTP requests.  The provided
CLI and TUI applications allow for exploring and sending these requests to a
HTTP server.  This makes it easier to test, document, and debug APIs defined in
markdown.

### ✨ Added

- CI/CD workflows for automated testing and releases
- Repository information to each packages metadata
- Windows installer (MSI) configuration for CLI and TUI binaries
- Initial CHANGELOG documenting the 0.1.1 release
