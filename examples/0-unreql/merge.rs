use serde::Deserialize;
use unreql::{func, r, rjson};

use unreql_examples::connect_opts;

#[tokio::main]
async fn main() {
    let sess = r.connect(connect_opts()).await.unwrap();

    let users: Vec<User> = r
        .table("users")
        .limit(10)
        .merge(func!(|user| {
            rjson!({
                "files": r.table("files")
                    .get_all(r.with_opt(user.g("id"), r.index("user_id")))
                    .limit(10)
                    .coerce_to("array")
            })
        }))
        .exec_to_vec(&sess)
        .await
        .unwrap();

    dbg!(users);
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User {
    id: String,
    files: Vec<File>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct File {
    id: String,
    name: Option<String>,
}
