# Library Layout

The heart of the ReqMd project is a Rust workspace containing crates which
separate the responsibilities into smaller distinct features.  Not every
crate is completely independent; however, some of the crates depend on one
or more of the `reqmd_` crates to provide their own library or binary.  These
are all located in the `crates/` directory in the root of this repository.

```text
crates/
  |- reqmd_app/
  |- reqmd_ast/
  |- reqmd_cli/
  |- reqmd_core/
  |- reqmd_explore/
  |- reqmd_http/
```

## High Level Overview

- `reqmd_app`

  This is a library encapsulation of a high level interface interface useful
  for application development.  It provides functionality for parsing documents
  into request structures.  These structures can then be sent off through the
  same interface.

- `reqmd_ast`

  Provides data structures and functionality for parsing markdown strings.

- `reqmd_cli`

  Binary crate which serves as a command line tool for working with markdown
  files that have [formatted requests] defined in them.

  [formatted requests]: ./reqmd-format.md

- `reqmd_core`

  Defines a set of domain level structures and a factory building interfaces
  to extend the creation of these structures.

- `reqmd_explore`

  Experimental library and TUI binary for working with ReqMd markdown files.

- `reqmd_http`

  Provides a data structure representation of HTTP requests and responses as
  well as an client interface to send and receive them.
