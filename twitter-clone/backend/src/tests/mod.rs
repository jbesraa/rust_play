#[allow(unused_imports)]
mod test_helpers;

use test_helpers::*;

#[async_std::test]

async fn empty_users_list() {
    let mut server = test_server().await;
    let test_db = TestDb::new().await;
    let db_pool = test_db.db();
    // let mut server = test_server().await;
    // let res = Request::build().get().url("/users").send(&mut server);
    // assert_eq!(res.status(), 200);

    // let json = res.body_json::<Value>().await.unwrap();
    // assert_json_eq!(json, json!([]));
}

// #[async_std::test]
// async fn creating_a_user() {
//     let mut server = test_server().await;
//     let res = Request::build().get().url("/users").send(&mut server);
//     assert_eq!(res.status(), 200);
//     let json: Value = res.body_json::<Value>().await.unwrap();
//     dbg!(&json);
//     assert_json_eq!(json, json!([]));

// let url = Url::parse("http://example.com/users").unwrap();
// let mut req = Request::new(Method::Post, url);
// req.set_body(json!({"username": "ewq"}).to_string());
// req.set_content_type("application/json".parse().unwrap());
// let res = server.simulate(req).unwrap();
// assert_eq!(res.status(), 201);

// let res = Request::build().get().url("/users").send(&mut server);
// assert_eq!(res.status(), 200);
// let json = res.body_json::<Value>().await.unwrap();
// assert_json_include!(actual: json, expected: json!([{"username": "bob"}]));
// }

// async fn first_test() {
//     let mut server = test_server().await;
//     let res = Request::build().get().url("/").send(&mut server);
//     assert_eq!(res.status(), 200);
//     let json: Value = res.body_json().await.unwrap();
//     assert_json_include!(actual: json, expected: json!([1, 2, 3]))
// }
