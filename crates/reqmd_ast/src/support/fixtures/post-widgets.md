---
http:
  headers:
    - key: Content-Type
      value: application/json
---

# Post Widgets

I've often wondered what this text is called

```http
POST /widgets
     ?foo=bar
     &rofl=copter
Authorization: Bearer abcd1234
```

```json http-body
{
  "name": "XFox",
  "desc": "Wonderful widget!"
}
```
