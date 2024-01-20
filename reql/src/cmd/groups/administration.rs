use unreql_macros::create_cmd;
use ql2::term::TermType;
use serde::Serialize;

use crate::{
    cmd::{
        args::{ManyArgs, Opt},
        options::{GrantOptions, ReconfigureOptions, WaitOptions},
    },
    Command,
};

create_cmd!(
    /// Grant or deny access permissions for a user account, globally or
    /// on a per-database or per-table basis.
    ///
    /// See details in [javascript documentation](https://rethinkdb.com/api/javascript/grant).
    grant(username: Serialize, opts: Opt<GrantOptions>)
);

create_cmd!(
    /// Query (read and/or update) the configurations for individual tables or databases.
    ///
    /// See details in [javascript documentation](https://rethinkdb.com/api/javascript/config).
    only_command,
    config
);

create_cmd!(
    /// Rebalances the shards of a table. When called on a database, all
    /// the tables in that database will be rebalanced.
    ///
    /// See details in [javascript documentation](https://rethinkdb.com/api/javascript/rebalance).
    only_command,
    rebalance
);

create_cmd!(
    /// Reconfigure a table’s sharding and replication.
    ///
    /// See details in [javascript documentation](https://rethinkdb.com/api/javascript/reconfigure).
    only_command,
    reconfigure(args: ManyArgs<ReconfigureOptions>)
);

create_cmd!(
    /// Return the status of a table.
    ///
    /// See details in [javascript documentation](https://rethinkdb.com/api/javascript/status).
    only_command,
    status
);

create_cmd!(
    /// Wait for a table or all the tables in a database to be ready
    ///
    /// See details in [javascript documentation](https://rethinkdb.com/api/javascript/wait).
    only_root,
    wait(table_or_database: Serialize, opts: Opt<WaitOptions>)
    only_command,
    wait(opts: Opt<WaitOptions>)
);
