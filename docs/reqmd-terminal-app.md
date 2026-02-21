# 🖥️ **`reqmd`** CLI

`reqmd` is a terminal application which acts as both a parser of
properly formatted documents and a runner of the HTTP requests
contained within those documents.  This tool is designed to be
used in a variety of ways, from a simple http client to a more
complex part of a CI/CD pipeline, and also serves as a reference
implementation for API documentation written in the ReqMd format.

---

**The tl;dr is to be able to take a document like this:**

> Sample document:
> ![Sample Document](./assets/sample-doc.png)

**And run commands like this:**

---

> List requests found in the document:
> ![List Requests](./assets/list-sample.png)

---

> [!Note]
>
> The server in the example is an echo server that replies with
> the information it was sent.  What you are seeing returned
> is the response of that echo server.  This helps demonstrate
> what the details were in the requests this tool sent.

---

> Send first request in the document:
> ![Send First Request](./assets/send-first-request.png)

---

> Adds header to request with environment variable:
> ![Environment Headers](./assets/env-header-example.png)

---

> Can set timeouts _( Examples: 50ms, 2sec, 5min )_
> ![Error Timeout](./assets/error-timeout.png)

---

> Run by request found at a line number:
> ![Run Line 18](./assets/run-line-18.png)

---

> Dump requests to JSON:
> ![Dump to JSON](./assets/dump-to-json.png)
