use futures::TryStreamExt;
use serde_json::Value;
use unreql::{cmd::options::RandomOptions, r};

#[tokio::test]
async fn random_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.random(()).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    dbg!(val);
    Ok(())
}

#[tokio::test]
async fn random_one_arg_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.random(100).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    dbg!(val);
    Ok(())
}

#[tokio::test]
async fn random_one_arg_with_opts_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let opts = RandomOptions { float: Some(true) };
    let mut query = r.random(r.with_opt(100, opts)).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    dbg!(val);
    Ok(())
}

#[tokio::test]
async fn random_two_args_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let mut query = r.random(r.args([100, 200])).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    dbg!(val);
    Ok(())
}

#[tokio::test]
async fn random_two_args_with_opts_query() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let opts = RandomOptions { float: Some(true) };
    let mut query = r.random(r.with_opt(r.args([100, 200]), opts)).run(&conn);
    let val: Option<Value> = query.try_next().await?;
    assert!(val.is_some());
    dbg!(val);
    Ok(())
}
