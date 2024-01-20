use futures::TryStreamExt;
use serde_json::Value;
use unreql::r;

#[tokio::test]
async fn expr_not_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr(true).not().run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(Value::Bool(false)));
    Ok(())
}

#[tokio::test]
async fn expr_not_not_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr(true).not().not().run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(Value::Bool(true)));
    Ok(())
}
