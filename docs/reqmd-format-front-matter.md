# 🔎 Front Matter

## 🔬 Details

Front matter is an extension to markdown which provides metadata information
to a document that is not intended to be directly rendered. It is a YAML
encoded hash at the very top of the document that is sandwiched between sets
of `---`.  ReqMd leverages this extension to extract the following three data
points:

`title`
: Optional string attached to the AST metadata which is expected
to be a short, single line identification of the document.  If
not provided this defaults to `null` in JSON or `None` in Rust if
using the respective libraries.

`description`
: Optional string attached to the AST metadata which can be multiple
lines long an is a synopsis of the document.  If not provided this
defaults to `null` in JSON or `None` just like the title.

`http`
: Optional hash structure which provides default values to all http
requests defined in the same document.  This hash has itself three
keys all of which are optional: `server`, `headers`, and `query`; each
of which is described below as `http.{key}`.

`http.server`
: This is an URL encoded string which is the base of a HTTP address.
This includes either `http` or `https`, the DNS or IP address, and
optional port number.  Example: `https://example.com:8080`.  This
defaults to `http://localhost` if not provided.

`http.headers`
: An array of hashes with two keys of `key` and `value`.  This is an
array to allow for multiple headers with the same key to be defined
should you need to do so.  Defaults to an empty array if not
provided.

`http.query`
: Similar to headers, this is an array of hashes with two keys of
`key` and `value`.  Also just like the headers this defaults to
an empty array if not provided.

## 📖 Markdown Example

```text
---
title: Example ReqMd Document
description: |
  This is an example of a ReqMd document with front matter.  The
  description field can be multiple lines long and is a synopsis
  of the document.
http:
  server: https://example.com:8080
  headers:
    - key: Content-Type
      value: application/json
    - key: Accept
      value: application/json
  query:
    - key: foo
      value: bar
---
```
