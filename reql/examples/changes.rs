use futures::TryStreamExt;
use serde_json::Value;
use unreql::{cmd::options::ChangesOptions, r};

use crate::shared::connect_opts;

mod shared;

#[tokio::main]
async fn main() {
    let sess = r.connect(connect_opts()).await.unwrap();

    let opts = ChangesOptions::new()
        .include_initial(true)
        .include_states(true);
    let mut q = r.table("test").changes(opts).run::<_, Value>(&sess);

    while let Ok(Some(changed)) = q.try_next().await {
        dbg!(changed);
    }

    println!("done");
}
