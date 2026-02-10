use unreql::{cmd::options::RunOptions, r};

use unreql_examples::connect_opts;

#[tokio::main]
async fn main() {
    let sess = r.connect(connect_opts()).await.unwrap();

    // Error: "Array over size limit `5`"
    // Cause: .expr contains 10 elements, exceeding the 5-element limit
    //let opts = RunOptions::new().array_limit(5);

    // Without error
    let opts = RunOptions::new().array_limit(15);

    let nums: Vec<i64> = r
        .expr(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        // Pass opts to .run (or its equivalent) via the r.args method.
        .exec_to_vec(r.args((&sess, opts)))
        .await
        .unwrap();

    dbg!(nums);
}
