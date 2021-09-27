HOST_ONE: http://localhost:4000
HOST_TWO: http://localhost:8080

```json SOME_PAYLOAD
{
  "test": "it"
}
```

## Request One

```
GET /blogs/2
Host: $HOST_ONE
```


## Request Two

```
POST /blogs
Host: $HOST_TWO
Content-Type: application/json
```
```json
{
  "title": "Sample Title",
  "body": "Sample Body"
}
```
