# Sample Environment Variable Expansion

This will expand variables from the environment.

```http
POST /api/$API_VERSION/users?debug=$DEBUG_MODE
X-API-KEY: $API_KEY
```

```json
{ "first_name": "$FIRST_NAME" }
```
