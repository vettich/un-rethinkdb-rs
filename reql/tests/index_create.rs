use futures::TryStreamExt;
use serde_json::Value;
use unreql::{func, r};

#[tokio::test]
async fn index_create() -> unreql::Result<()> {
    tracing_subscriber::fmt::init();

    let conn = r.connect(()).await?;

    let _ = r
        .table_create("comments")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table("comments")
        .index_drop("author_name")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table("comments")
        .index_create(r.args(("author_name", func!(|doc| doc.g("author").g("name")))))
        .run::<_, Value>(&conn)
        .try_next()
        .await?;

    let _ = r
        .table("comments")
        .index_drop("post_and_date")
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table("comments")
        .index_create(r.args((
            "post_and_date",
            func!(|doc| [doc.clone().g("post_id"), doc.g("date")]),
        )))
        .run::<_, Value>(&conn)
        .try_next()
        .await?;

    Ok(())
}
