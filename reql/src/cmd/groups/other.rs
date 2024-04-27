use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{cmd::options::Index, r, rjson, Command};

create_cmd!(minval);
create_cmd!(maxval);

create_cmd!(asc(key: Serialize));
create_cmd!(desc(key: Serialize));

create_cmd!(
    /// Array
    array:MakeArray,
    Serialize
);

impl r {
    pub fn index(self, arg: impl Serialize + 'static) -> Index {
        let obj = rjson!({
            "index": arg
        });
        Index(obj)
    }
}
