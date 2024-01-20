use serde_json::to_string;
use unreql::{r, rjson};

#[tokio::test]
async fn count_query() -> unreql::Result<()> {
    let query = r.table("table").filter(rjson!({ "id": 103 })).count(());
    assert_eq!(
        r#"[43,[[39,[[15,["table"]],{"id":103}]]]]"#,
        to_string(&query).unwrap()
    );
    Ok(())
}
