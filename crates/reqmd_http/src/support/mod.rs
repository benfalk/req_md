pub mod fixtures {
    use crate::request::{Method, Request, RequestFactory};
    use ::rstest::fixture;
    use ::url::Host;

    #[fixture]
    fn example_host() -> Host {
        Host::parse("example.com").expect("Failed to parse host")
    }

    #[fixture]
    fn example_factory() -> RequestFactory {
        Request::factory()
            .address_port(8080)
            .method(Method::Get)
            .header("Accept", "application/json")
            .build()
    }
}
