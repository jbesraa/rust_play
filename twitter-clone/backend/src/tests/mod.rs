#[allow(unused_imports)]
mod test_helpers;

use test_helpers::*;

#[async_std::test]
async fn creating_a_user() {
    let mut server = test_server().await;
    let res = Request::build().get().url("/users").send(&mut server);
    assert_eq!(res.status(), 200);
    let json: Value = res.body_json::<Value>().await.unwrap();
    assert_json_include!(actual: json, expected: json!([]));
}

// async fn first_test() {
//     let mut server = test_server().await;
//     let res = Request::build().get().url("/").send(&mut server);
//     assert_eq!(res.status(), 200);
//     let json: Value = res.body_json().await.unwrap();
//     assert_json_include!(actual: json, expected: json!([1, 2, 3]))
// }
