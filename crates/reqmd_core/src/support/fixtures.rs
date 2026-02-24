use crate::File;

#[rstest::fixture]
pub fn sample_request_file() -> File {
    File::load(include_str!("./fixtures/sample-request.md"), None).unwrap()
}

#[rstest::fixture]
pub fn sample_env_var_expansion_request_file() -> File {
    File::load(include_str!("./fixtures/sample-env-expansion.md"), None).unwrap()
}

#[rstest::fixture]
pub fn sample_server_from_hostname_request_file() -> File {
    File::load(
        include_str!("./fixtures/sample-server-from-hostname.md"),
        None,
    )
    .unwrap()
}

#[cfg(feature = "yaml-as-json")]
#[rstest::fixture]
pub fn sample_yaml_as_json_request_file() -> File {
    File::load(include_str!("./fixtures/sample-yaml-as-json.md"), None).unwrap()
}
