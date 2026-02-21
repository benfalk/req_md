# 📐 ReqMd Markdown Format

ReqMd is designed to leverage already existing markdown features and
conventions as much as possible.  This means that ReqMd documents are
valid to render in any other context.  This also means that ReqMd
documents can be rendered in any system and will gracefully degrade if
the system does not support all of the ReqMd features.

This example highlights a sample document which can be [downloaded]
from the assets directory.  The top of the document is a header following
the front matter convention and describes the server to send requests to.
After this section are three HTTP sections showcasing some of the requests
that can be fashioned.

````text
{{#aa ./assets/sample.md}}
````

For more details on the format see the following pages:

- [🔎 Front Matter](./reqmd-format-front-matter.md)
- [🌐 HTTP Requests](./reqmd-format-http-requests.md)

[downloaded]: https://github.com/benfalk/req_md/blob/master/docs/assets/sample.md
