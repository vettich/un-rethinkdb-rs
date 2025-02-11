use unreql::{func, r, rjson};

use unreql_examples::connect_opts;

#[tokio::main]
async fn main() {
    let sess = r.connect(connect_opts()).await.unwrap();

    // Using .merge() for a single object with a case when none exists
    let result: serde_json::Value = r
        .table("test")
        .get("test_id")
        .do_(func!(|doc| {
            // check if doc is exist
            doc.clone().branch(
                // if exist then merge
                doc.merge(rjson!({
                    // build other object fields
                    "new_field": true,
                })),
                // if not exist then return null
                rjson!(null),
            )
        }))
        .exec(&sess)
        .await
        .unwrap();

    dbg!(result);
}
