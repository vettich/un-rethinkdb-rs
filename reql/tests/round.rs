use futures::TryStreamExt;
use serde_json::{json, Value};
use unreql::r;

#[tokio::test]
async fn round_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr(12.345).round().run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(json!(12)));
    Ok(())
}

#[tokio::test]
async fn round_with_arg_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.round(-12.345).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(json!(-12)));
    Ok(())
}
