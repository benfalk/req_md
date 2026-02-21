# 🕹️ Commands

> [!NOTE]
>
> For every example that follows the [sample.md] document was used
> for input.  Refer to it for more context or try out the examples
> for yourself.

[sample.md]: https://raw.githubusercontent.com/benfalk/req_md/refs/heads/master/docs/assets/sample.md

## ❓ **`reqmd help`**

Produces the following output:

```text
Tool for sending HTTP requests defined in markdown files

Usage: reqmd <COMMAND>

Commands:
  list  Lists all of the requests found in order
  send  Sends request from file to server
  dump  Outputs JSON representation of parsed requests
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## 📝 **`reqmd list`**

Parses a document and lists all of the requests found in order.
If the request has a matching header it is displayed in the
list; however, if none is found the request line is used instead.

```bash
reqmd list docs/assets/sample.md
```

```text
1. Create Widget
2. Delete Widget
3. Search Widgets
```

## 🚀 **`reqmd send`**

Sends specified request from file to server.  This main parameter
is the filename followed by a colon and a selection identifier.
Currently the selection identifiers are:

- `:{n}` where `n` is a 1-based index of the request from the file
- `:first` the first found request, same as `:1`
- `:last` the last request in the document.
- `:line{n}` where `n` is a 1-based line number of the document.
  The request that contains this line number is selected.

The response is printed back as long as the value is valid UTF-8.

### 📖 Second Request

```bash
reqmd send docs/assets/sample.md:2
```

```text
{
  "method": "DELETE",
  "protocol": "https",
  "host": "echo.free.beeceptor.com",
  "path": "/widget/123",
  "ip": "[2605:59ca:6787:b808:8141:5500:1044:65b6]:40516",
  "headers": {
    "Host": "echo.free.beeceptor.com",
    "Accept": "*/*",
    "Via": "2.0 Caddy",
    "Accept-Encoding": "gzip"
  },
  "parsedQueryParams": {}
}
```

### 📖 Last Request

```bash
reqmd send docs/assets/sample.md:last
```

```text
{
  "method": "GET",
  "protocol": "https",
  "host": "echo.free.beeceptor.com",
  "path": "/widget/search?q=full%2Bmetal&max=10",
  "ip": "[2605:59ca:6787:b808:8141:5500:1044:65b6]:41236",
  "headers": {
    "Host": "echo.free.beeceptor.com",
    "Accept": "*/*",
    "Via": "2.0 Caddy",
    "Accept-Encoding": "gzip"
  },
  "parsedQueryParams": {
    "q": "full+metal",
    "max": "10"
  }
}
```

### 📖 On Line 10

```bash
reqmd send docs/assets/sample.md:line10
```

```text
{
  "method": "POST",
  "protocol": "https",
  "host": "echo.free.beeceptor.com",
  "path": "/widget",
  "ip": "[2605:59ca:6787:b808:8141:5500:1044:65b6]:56286",
  "headers": {
    "Host": "echo.free.beeceptor.com",
    "Content-Length": "17",
    "Accept": "*/*",
    "Content-Type": "application/json",
    "Via": "2.0 Caddy",
    "Accept-Encoding": "gzip"
  },
  "parsedQueryParams": {},
  "parsedBody": {
    "name": "foo"
  }
}
```

## ⚙️ **`reqmd dump`**

Returns a JSON representation of the parsed requests from the document for
other tools to consume.

```bash
reqmd dump docs/assets/sample.md
```

```json
[
  {
    "title": "Create Widget",
    "description": null,
    "request": {
      "address": {
        "host": {
          "Domain": "echo.free.beeceptor.com"
        },
        "scheme": "Https",
        "port": null
      },
      "method": "Post",
      "path": "/widget",
      "query": [],
      "headers": [
        {
          "key": "Content-Type",
          "value": "application/json"
        }
      ],
      "body": {
        "Text": "{ \"name\": \"foo\" }"
      }
    },
    "data": {
      "title": "Create Widget",
      "description": null,
      "method": "Post",
      "path": "/widget",
      "query": [],
      "headers": [
        {
          "key": "Content-Type",
          "value": "application/json"
        }
      ],
      "body": {
        "content": {
          "Text": "{ \"name\": \"foo\" }"
        },
        "lang": "json",
        "meta": null,
        "position": {
          "start": {
            "line": 14,
            "column": 1,
            "offset": 160
          },
          "end": {
            "line": 16,
            "column": 4,
            "offset": 189
          }
        }
      },
      "position": {
        "start": {
          "line": 7,
          "column": 1,
          "offset": 85
        },
        "end": {
          "line": 16,
          "column": 4,
          "offset": 189
        }
      }
    }
  },
  {
    "title": "Delete Widget",
    "description": null,
    "request": {
      "address": {
        "host": {
          "Domain": "echo.free.beeceptor.com"
        },
        "scheme": "Https",
        "port": null
      },
      "method": "Delete",
      "path": "/widget/123",
      "query": [],
      "headers": [],
      "body": "None"
    },
    "data": {
      "title": "Delete Widget",
      "description": null,
      "method": "Delete",
      "path": "/widget/123",
      "query": [],
      "headers": [],
      "body": {
        "content": "None",
        "lang": null,
        "meta": null,
        "position": null
      },
      "position": {
        "start": {
          "line": 18,
          "column": 1,
          "offset": 191
        },
        "end": {
          "line": 22,
          "column": 4,
          "offset": 239
        }
      }
    }
  },
  {
    "title": "Search Widgets",
    "description": null,
    "request": {
      "address": {
        "host": {
          "Domain": "echo.free.beeceptor.com"
        },
        "scheme": "Https",
        "port": null
      },
      "method": "Get",
      "path": "/widget/search",
      "query": [
        {
          "key": "q",
          "value": "full+metal"
        },
        {
          "key": "max",
          "value": "10"
        }
      ],
      "headers": [],
      "body": "None"
    },
    "data": {
      "title": "Search Widgets",
      "description": null,
      "method": "Get",
      "path": "/widget/search",
      "query": [
        {
          "key": "q",
          "value": "full+metal"
        },
        {
          "key": "max",
          "value": "10"
        }
      ],
      "headers": [],
      "body": {
        "content": "None",
        "lang": null,
        "meta": null,
        "position": null
      },
      "position": {
        "start": {
          "line": 24,
          "column": 1,
          "offset": 241
        },
        "end": {
          "line": 30,
          "column": 4,
          "offset": 320
        }
      }
    }
  }
]
```
