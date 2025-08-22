//
// cargo run --example http-example --features "serde,reqwest"
//

use ::reqmd_http::{Error, Host, Method, Request, Scheme};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let host = Host::parse("echo.free.beeceptor.com").expect("valid host");

    // Create a new request
    let request = Request::builder()
        .method(Method::Post)
        .address_scheme(Scheme::Https)
        .address_host(host)
        .path("/widget")
        .header("Content-Type", "application/json")
        .body_text(r#"{"name":"foo"}"#)
        .build();

    // What does it look like in JSON?
    #[cfg(feature = "serde")]
    {
        let json_req = ::serde_json::to_value(request.clone()).unwrap();
        println!("{json_req:#?}");
    }

    #[cfg(not(feature = "serde"))]
    {
        // If serde is not enabled, we can still print the request
        println!("{:#?}", &request);
    }

    // Reqwest client implements [reqmd_http::client::Client]
    #[cfg(feature = "reqwest")]
    {
        use ::reqmd_http::Client as _;
        use ::reqwest::Client;

        // Create a new Reqwest client
        let client = Client::new();

        // Utilize the client to send the request
        let response = client.send(&request).await?;

        // What does the response look like in JSON?
        #[cfg(feature = "serde")]
        {
            let json_resp = ::serde_json::to_value(response).unwrap();
            println!("{json_resp:#?}");
        }

        #[cfg(not(feature = "serde"))]
        {
            // If serde is not enabled, we can still print the response
            println!("{:#?}", &response);
        }
    }

    Ok(())
}
