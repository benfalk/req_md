# Request MD

Converts a request formatted with a markdown format
into a http request.

For instance the following request is a valid req_md
markdown request:

```
POST /widgets
Content-Type: application/json
Host: localhost:3000
```
```json
{
  "name": "foo",
  "type": "mark-1",
  "series": 8
}
```

As of right now this is pretty primitive; but as I get
time I'll add features to it.  Currently it will only
process what is piped to it's stdin.
