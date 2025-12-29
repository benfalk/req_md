# Sample `YAML` as `JSON`

This allows you to send a `YAML` payload as `JSON` in an HTTP request.

```http
POST /api/users
Content-Type: application/json
```

```yaml send-as-json
first_name: John
last_name: Doe
age: 22
active: true
hobbies:
  - reading
  - hiking
  - coding
```
