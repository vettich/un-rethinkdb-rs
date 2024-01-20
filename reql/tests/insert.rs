use serde_json::to_string;
use unreql::{
    cmd::options::{Conflict, InsertOptions},
    r, rjson,
};

#[tokio::test]
async fn insert_json() -> unreql::Result<()> {
    let cmd = r.table("table").insert(rjson!({ "value": true }));
    assert_eq!(
        r#"[56,[[15,["table"]],{"value":true}]]"#,
        to_string(&cmd).unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn insert_many() -> unreql::Result<()> {
    let cmd = r
        .table("table")
        .insert(r.args([rjson!({ "doc1": true }), rjson!({ "doc2": true })]));
    assert_eq!(
        r#"[56,[[15,["table"]],{"doc1":true},{"doc2":true}]]"#,
        to_string(&cmd).unwrap()
    );
    Ok(())
}

#[tokio::test]
async fn insert_with_opt() -> unreql::Result<()> {
    let cmd = r.table("table").insert(r.with_opt(
        rjson!({"value": true}),
        InsertOptions::new().conflict(Conflict::Update),
    ));
    assert_eq!(
        r#"[56,[[15,["table"]],{"value":true}],{"conflict":"update"}]"#,
        to_string(&cmd).unwrap()
    );
    Ok(())
}
