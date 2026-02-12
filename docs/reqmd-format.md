# ReqMd Markdown Format

## Front Matter Specification

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

### Front Matter Example

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

## Code Block Specification

Requests are defined in markdown code blocks with the language tag of `http`.
These code blocks do not strictly follow the HTTP specification but are instead
a more flexible format that is more readable and writable by humans.  The start
of the code block is the method and path of the request.  The method is one of
the standard HTTP methods such as `GET`, `POST`, `PUT`, etc.  The path is the
string that comes after the server in a URL.  This can include query parameters
and this is where the format deviates from the standard HTTP specification.
Each query parameter can be on it's own line and they can be separated by either
`?` or `&`.  Neither the key or value parts of the query parameters need to be
URL encoded.  The rest of the code block is the headers of the request.  These
are in the standard HTTP format of `Key: Value` and can be as many as needed.

Body content is defined in a separate code block as long as the language tag is
not `http`.  It is important to note that nothing but white exists between the
two code blocks.  This is to allow for the body content to be in any format but
still be associated with the request.  The language tag of the body content is
to allow for syntax highlighting and is not used by ReqMd for any other purpose.

In addition to this in the AST there is a `title` and `description`.  These are
both optional and are identified by the first header found above the `http`
code block.  Any content between this header and the code block is considered
to be the description.  The title is the content of the header itself.  If no
text is found between the header and the code block then the description is
`null` in JSON or `None` in Rust.

### Code Block Example

````text
## Title of the Request

This is the description of the request.  It can be multiple lines long and
have **any** of the standard markdown formatting.  It is preserved exactly
as it is in the source as a string in the AST metadata.

```http
POST /widgets?foo=bar
             &fizz=buzz
Content-Type: application/json
```

```json
{
  "name": "XFox",
  "desc": "Wonderful widget!"
}
```
````
