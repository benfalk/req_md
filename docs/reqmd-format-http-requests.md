# 🌐 HTTP Requests

## 🔬 Details

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

## 📖 Markdown Example

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
