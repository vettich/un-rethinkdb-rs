use futures::TryStreamExt;
use serde_json::Value;
use unreql::r;

#[tokio::test]
async fn expr_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr(r.expr("hello")).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(Value::String("hello".into())));
    Ok(())
}
