# 🔌 Builtin Plugins

The core of `reqmd` supports extending how it parses markdown documents
and this CLI enables several that ship with the library.  In no particular
order, these include:

- Env Var Overrides & Additions
- Env Var Substitution
- YAML Body to JSON Body Conversion

When `reqmd` is run it will attempt to load a `.env` file from the current
working directory and apply any variables defined there as environment
variables for plugins.

## 🎛️ ENV Var HTTP Settings

In addition to the front matter of a markdown document, `reqmd` allows for
you to supply environment variables which will apply to all requests parsed.

- `REQMD_SERVER`

  If set, this will override the server URL for all requests in the document.

- `REQMD_QUERY_{parm}`

  For each environment variable that starts with `REQMD_QUERY_`, the remainder
  of the variable name will be used as a query parameter name and the value of
  the variable will be used as the value for that query parameter.  As an
  example, the query parameter `?foo=bar` can be set for all requests in the
  document with `REQMD_QUERY_foo=bar`.

- `REQMD_HEADER_{header}`

  For each environment variable that starts with `REQMD_HEADER_`, the remainder
  of the variable name will be used as a header name and the value of the
  variable will be used as the value for that header.  As an example, the
  header `Foo: bar` can be set for all requests in the document with
  `REQMD_HEADER_Foo=bar`.

## 🔃 ENV Var Substitution

As the headers, query parameters, and body of a request are parsed, any value
that starts with `$` will be substituted with a variable from the environment.
If an environment variable is not found the dollar sign and name are left
as-is.

## 🫂 YAML to JSON Body

Sometimes it can be easier to write request bodies in YAML format, but the API
you are working with may only accept JSON.  If the body of a request is marked
with a language tag of `yml` or `yaml` and has a meta tag of `send-as-json` the
body will be parsed as YAML and then converted to JSON before being sent.
