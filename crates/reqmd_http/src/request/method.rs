/// HTTP defines a set of request methods to indicate the purpose of
/// the request and what is expected if the request is successful.
/// Although they can also be nouns, these request methods are sometimes
/// referred to as HTTP verbs. Each request method has its own semantics,
/// but some characteristics are shared across multiple methods.
///
/// These characteristics include:
///
/// - `safe`:
///   A request method is considered safe if it doesn't
///   alter the state of the server
///
/// - `idempotent`:
///   A request method is considered idempoten if the
///   intended effect on the server of making a single
///   request is the same as the effect of making several
///   identical requests
///
/// - `cacheable`:
///   not all request methods can be cached per the specification
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Method {
    /// The GET HTTP method requests a representation of the specified
    /// resource. Requests using GET should only be used to request data
    /// and shouldn't contain a body.
    #[default]
    Get,

    /// The POST HTTP method sends data to the server. The type of the
    /// body of the request is indicated by the `Content-Type` header.
    Post,

    /// The PUT HTTP method creates a new resource or replaces a
    /// representation of the target resource with the request content.
    /// The difference between PUT and POST is that PUT is idempotent:
    /// calling it once is no different from calling it several times
    /// successively (there are no side effects).
    Put,

    /// The DELETE HTTP method asks the server to delete a specified
    /// resource. The DELETE method has no defined semantics for the
    /// message body, so this should be empty.
    Delete,

    /// The PATCH HTTP method applies partial modifications to a resource.
    ///
    /// PATCH is somewhat analogous to the "update" concept found in CRUD
    /// > In general, HTTP is different than CRUD, and the two should not
    /// > be confused.
    ///
    /// In comparison with PUT, a PATCH serves as a set of instructions
    /// for modifying a resource, whereas PUT represents a complete
    /// replacement of the resource. A PUT request is always idempotent
    /// (repeating the same request multiple times results in the resource
    /// remaining in the same state), whereas a PATCH request may not
    /// always be idempotent. For instance, if a resource includes an
    /// auto-incrementing counter, a PUT request will overwrite the
    /// counter (since it replaces the entire resource), but a PATCH
    /// request may not.
    Patch,

    /// The HEAD HTTP method requests the metadata of a resource in the
    /// form of headers that the server would have sent if the GET method
    /// was used instead. This method can be used in cases where a URL
    /// might produce a large download, for example, a HEAD request can
    /// read the Content-Length header to check the file size before
    /// downloading the file with a GET.
    Head,

    /// The CONNECT HTTP method requests that a proxy establish a HTTP
    /// tunnel to a destination server, and if successful, blindly
    /// forward data in both directions until the tunnel is closed.
    Connect,

    /// The OPTIONS HTTP method requests permitted communication options
    /// for a given URL or server. This can be used to test the allowed
    /// HTTP methods for a request, or to determine whether a request
    /// would succeed when making a CORS preflighted request. A client
    /// can specify a URL with this method, or an asterisk (*) to refer
    /// to the entire server.
    Options,

    /// The TRACE HTTP method performs a message loop-back test
    /// along the path to the target resource.
    Trace,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
            Method::Head => "HEAD",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
        }
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ::std::str::FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "PATCH" => Ok(Self::Patch),
            "HEAD" => Ok(Self::Head),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            _ => Err(()),
        }
    }
}
