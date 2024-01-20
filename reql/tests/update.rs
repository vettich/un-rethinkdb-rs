use serde_json::{json, to_string};
use unreql::{r, rjson};
use unreql_macros::func;

#[tokio::test]
async fn update_json() -> unreql::Result<()> {
    let cmd = r.table("table").get("id").update(json!({ "value": true }));
    assert_eq!(
        r#"[53,[[16,[[15,["table"]],"id"]],{"value":true}]]"#,
        to_string(&cmd).unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn update_json_with_row() -> unreql::Result<()> {
    let cmd = r
        .table("table")
        .get("id")
        .update(rjson!({ "value": r.row().g("old_value") }));

    assert_eq!(
        r#"[53,[[16,[[15,["table"]],"id"]],[69,[[2,[1]],{"value":[31,[[13],"old_value"]]}]]]]"#,
        to_string(&cmd).unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn update_json_with_func_row() -> unreql::Result<()> {
    let cmd = r
        .table("table")
        .get("id")
        .update(func!(|doc| rjson!({"value": doc.g("old_value")})));

    assert_eq!(
        r#"[53,[[16,[[15,["table"]],"id"]],[69,[[2,[1]],{"value":[31,[[10,[1]],"old_value"]]}]]]]"#,
        to_string(&cmd).unwrap()
    );
    Ok(())
}
