use super::*;
use ::rstest::fixture;
use ::url::Host;

#[fixture]
fn example_factory() -> RequestFactory {
    Request::factory()
        .address_host(Host::parse("example.com").expect("valid host"))
        .build()
}
