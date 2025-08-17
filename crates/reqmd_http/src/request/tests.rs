use super::*;
use crate::address::Scheme;
use ::rstest::fixture;
use ::url::Host;

#[fixture]
fn example_factory() -> RequestFactory {
    Request::factory(|builder| {
        builder.address(|addr| {
            addr.scheme(Scheme::Http)
                .host(Host::parse("example.com").expect("valid host"));
        })
    })
}
