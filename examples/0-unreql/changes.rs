use futures::TryStreamExt;
use unreql::{cmd::options::ChangesOptions, r, types::Change};

use unreql_examples::connect_opts;

#[tokio::main]
async fn main() {
    let sess = r.connect(connect_opts()).await.unwrap();

    let opts = ChangesOptions::new()
        .include_initial(true)
        .include_states(true);
    let mut q = r.table("test").changes(opts).run::<_, Change>(&sess);

    while let Ok(Some(changed)) = q.try_next().await {
        dbg!(changed);
    }

    println!("done");
}
