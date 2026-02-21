---
title: Working with Widgets
http:
  server: https://echo.free.beeceptor.com
---

## Create Widget

```http
POST /widget
Content-Type: application/json
```

```json
{ "name": "foo" }
```

## Delete Widget

```http
DELETE /widget/123
```

## Search Widgets

```http
GET /widget/search
    ?q=full+metal
    &max=10
```
