use super::*;
use ::rstest::fixture;

#[fixture]
fn exaple_address() -> Address {
    Address::parse("https://example.com:8080").expect("valid address")
}

#[fixture]
fn example_factory(exaple_address: Address) -> RequestFactory {
    Request::factory()
fn example_address() -> Address {
    Address::parse("https://example.com:8080").expect("valid address")
}

#[fixture]
fn example_factory(example_address: Address) -> RequestFactory {
    Request::factory()
        .address(example_address)
        .header("User-Agent", "reqmd_http/0.1.0")
        .build()
}
