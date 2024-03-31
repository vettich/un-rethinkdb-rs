use std::time::Instant;

use deadpool::managed::Pool;
use unreql::r;
use unreql_deadpool::{IntoPoolWrapper, SessionManager};
use unreql_examples::connect_opts;

#[tokio::main]
async fn main() {
    let manager = SessionManager::new(connect_opts());
    let pool = Pool::builder(manager)
        .max_size(20)
        .build()
        .unwrap()
        .wrapper();
    const MAX: usize = 5000;

    let now = Instant::now();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<usize>(16);
    for i in 0..MAX {
        let pool = pool.clone();
        let tx_c = tx.clone();
        tokio::spawn(async move {
            // let sum = r.expr(1) + r.expr(2);
            let sum = r.expr(1).add(r.expr(2));
            let value: i32 = sum.exec(&pool).await.unwrap();
            assert_eq!(value, 3);
            tx_c.send(i).await.unwrap();
        });
    }
    for _ in 0..MAX {
        rx.recv().await.unwrap();
    }

    println!("cost: {:?}", now.elapsed());
}
