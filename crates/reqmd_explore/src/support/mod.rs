use ::reqmd_core::{HttpGroup, Id};
use ::rstest::{fixture, rstest};

#[fixture]
pub fn simple_req() -> HttpGroup {
    ::ron::from_str(include_str!("./simple.req.ron")).unwrap()
}

#[fixture]
pub fn simple_req_input_id() -> Id {
    "240aacc2-3348-47e4-94ac-e2ebdd6b1d9d".parse().unwrap()
}

#[rstest]
fn simple_req_fixture(simple_req: HttpGroup, simple_req_input_id: Id) {
    assert_eq!(simple_req.inputs.len(), 1);
    let req = simple_req.generate_request(&simple_req_input_id).unwrap();
    assert_eq!(req.method, ::reqmd_http::Method::Post);
    assert_eq!(
        req.address.build_url().as_str(),
        "https://echo.free.beeceptor.com/"
    );
    assert_eq!(req.path.as_str(), "/widget");
    assert_eq!(req.query.len(), 0);
    assert_eq!(req.body.text(), Some(r#"{"name":"foo"}"#));
}
