use futures::TryStreamExt;
use serde_json::{json, to_string, Value};
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

#[tokio::test]
async fn update_in_db() -> unreql::Result<()> {
    let conn = r.connect(()).await?;
    let table = "users_test";

    let _ = r
        .table_create(table)
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table(table)
        .get(1)
        .delete(())
        .run::<_, Value>(&conn)
        .try_next()
        .await;

    let _ = r
        .table(table)
        .insert(json! ({
            "id": 1,
            "name": "Ivan",
            "upd_count": 3,
        }))
        .run::<_, Value>(&conn)
        .try_next()
        .await?;

    let user = r
        .table(table)
        .get(1)
        .run::<_, Value>(&conn)
        .try_next()
        .await?;
    assert_eq!(user, Some(json!({"id": 1, "name": "Ivan", "upd_count": 3})));

    let _ = r
        .table(table)
        .get(1)
        .update(rjson! ({
            "name": "John",
            "upd_count": r.row().g("upd_count").add(1),
        }))
        .run::<_, Value>(&conn)
        .try_next()
        .await?;

    let user = r
        .table(table)
        .get(1)
        .run::<_, Value>(&conn)
        .try_next()
        .await?;
    assert_eq!(user, Some(json!({"id": 1, "name": "John", "upd_count": 4})));

    Ok(())
}
