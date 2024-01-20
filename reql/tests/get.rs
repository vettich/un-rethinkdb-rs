use futures::TryStreamExt;
use serde_json::Value;
use unreql::r;

#[tokio::test]
async fn get_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.db("test").table("test").get("test_id").run(&conn);
    let val: Option<Value> = query.try_next().await?;

    let expected = serde_json::json!({
        "id": "test_id",
        "value": "hello",
    });

    assert_eq!(val, Some(expected));
    Ok(())
}
