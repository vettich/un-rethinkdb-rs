use futures::TryStreamExt;
use serde_json::{json, Value};
use unreql::r;

#[tokio::test]
async fn add_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr(2).add(2).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(json!(4)));
    Ok(())
}

#[tokio::test]
async fn add_multi_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr(2).add(r.args([2, 2])).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(json!(6)));
    Ok(())
}

#[tokio::test]
async fn add_str_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr("bbb").add("aaa").run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(json!("bbbaaa")));
    Ok(())
}

#[tokio::test]
async fn add_array_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let query = r.expr(["foo", "bar"]).add(["buzz"]).run(&conn);
    let val: Vec<Value> = query.try_collect().await?;
    assert_eq!(Value::Array(val), json!(["foo", "bar", "buzz"]));
    Ok(())
}

#[tokio::test]
async fn add_args_array_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.expr(0).add(r.args([1, 2, 4])).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert_eq!(val, Some(json!(7)));
    Ok(())
}
