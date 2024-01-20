use serde_json::to_string;
use unreql::{cmd::options::FilterOptions, r, rjson};
use unreql_macros::func;

#[tokio::test]
async fn filter_query() -> unreql::Result<()> {
    let query = r.table("table").filter(rjson!({ "id": 103 }));
    assert_eq!(
        r#"[39,[[15,["table"]],{"id":103}]]"#,
        to_string(&query).unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn filter_by_row_query() -> unreql::Result<()> {
    let query = r.table("table").filter(r.row().g("id").eq("test_id"));
    assert_eq!(
        r#"[39,[[15,["table"]],[69,[[2,[1]],[17,[[31,[[13],"id"]],"test_id"]]]]]]"#,
        to_string(&query).unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn filter_by_func_query() -> unreql::Result<()> {
    let query = r
        .table("table")
        .filter(func!(|doc| doc.g("id").eq("test_id")));
    assert_eq!(
        r#"[39,[[15,["table"]],[69,[[2,[1]],[17,[[31,[[10,[1]],"id"]],"test_id"]]]]]]"#,
        to_string(&query).unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn filter_by_row_query_with_opt() -> unreql::Result<()> {
    let query = r.table("table").filter(r.with_opt(
        r.row().g("id").eq("test_id"),
        FilterOptions::new().default(true),
    ));
    assert_eq!(
        r#"[39,[[15,["table"]],[69,[[2,[1]],[17,[[31,[[13],"id"]],"test_id"]]]]],{"default":true}]"#,
        to_string(&query).unwrap()
    );
    Ok(())
}
