#[allow(unused_imports)]
mod test_helpers;

use assert_json_diff::assert_json_include;
use http_types::{Method, Request, Url};
use serde_json::{json, Value};
use test_helpers::*;

#[async_std::test]
async fn nested() {
    let mut server = test_server().await;
    let req = Request::new(Method::Get, Url::parse("http://example.com/").unwrap());
    let res = server.simulate(req).unwrap();

    assert_eq!(res.status(), 200);
    let json: Value = res.body_json().await.unwrap();

    assert_json_include!(actual: json, expected: json!([1, 2, 3]))
}
