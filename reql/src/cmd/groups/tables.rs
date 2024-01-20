use unreql_macros::create_cmd;
use ql2::term::TermType;
use serde::Serialize;

use crate::{
    cmd::{
        args::{Arg, ManyArgs, Opt},
        options,
    },
    Command,
};

create_cmd!(
    /// List all table names in a database. The result is a list of strings.
    ///
    /// ## Example
    /// List all tables of the ‘test’ database.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.db("test").table_list().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [table_create](Self::table_create)
    /// - [table_drop](Self::table_drop)
    table_list,
);

create_cmd!(
    /// Create a table. A RethinkDB table is a collection of JSON documents.
    ///
    /// If successful, the command returns an object with two fields:
    ///
    /// - `tables_created`: always `1`.
    /// - `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    ///   - `old_val`: always `null`.
    ///   - `new_val`: the table’s new [config](Command::config) value.
    ///
    /// If a table with the same name already exists, the command throws ReqlOpFailedError.
    ///
    /// *Note*: Only alphanumeric characters and underscores are valid for the table name.
    ///
    /// Invoking tableCreate without specifying a database using db creates a table in the database specified in connect, or test if no database was specified.
    ///
    ///When creating a table you can specify the following options:
    ///
    /// - `primaryKey`: the name of the primary key. The default primary key is `id`.
    /// - `durability`: if set to `soft`, writes will be acknowledged by the server immediately and flushed to disk in the background. The default is `hard`: acknowledgment of writes happens after data has been written to disk.
    /// - `shards`: the number of shards, an integer from 1-64. Defaults to `1`.
    /// - `replicas`: either an integer or a mapping object. Defaults to `1`.
    ///   - If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    ///   - If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}`.
    /// - `primaryReplicaTag`: the primary server specified by its server tag. Required if replicas is an object; the tag must be in the object. This must not be specified if replicas is an integer.
    ///
    /// The [data type](https://rethinkdb.com/docs/data-types/) of a primary key is usually a string
    /// (like a UUID) or a number, but it can also be a time, binary object, boolean or an array.
    /// Data types can be mixed in the primary key field, but all values must be unique.
    /// Using an array as a primary key causes the primary key to behave like a compound index;
    /// read the documentation on [compound secondary indexes](https://rethinkdb.com/docs/secondary-indexes/javascript/#compound-indexes) for more information,
    /// as it applies to primary keys as well. (Note that the primary index still only covers a single field,
    /// while compound secondary indexes can cover multiple fields in a single index.) Primary keys cannot be objects.
    ///
    /// ## Example
    /// Create a table named 'dc_universe' with the default settings.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.db("heroes").table_create("dc_universe").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a table named 'dc_universe' using the field 'name' as primary key.
    ///
    /// ```
    /// # use unreql::cmd::options::TableCreateOptions;
    /// # unreql::example(|r, conn| {
    /// let opts = TableCreateOptions::new().primary_key("name");
    /// r.db("test").table_create(r.with_opt("dc_universe", opts)).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a table set up for two shards and three replicas per shard. This requires three available servers.
    ///
    /// ```
    /// # use unreql::cmd::options::{TableCreateOptions, Replicas};
    /// # unreql::example(|r, conn| {
    /// let opts = TableCreateOptions::new()
    ///   .primary_key("name")
    ///   .replicas(Replicas::Int(3));
    /// r.db("test").table_create(r.with_opt("dc_universe", opts)).run(conn)
    /// # })
    /// ```
    ///
    /// Read [Sharding and replication](https://rethinkdb.com/docs/sharding-and-replication/) for a complete discussion of the subject, including advanced topics.
    ///
    /// # Related commands
    /// - [table_list](Self::table_list)
    /// - [table_drop](Self::table_drop)
    table_create(table_name:Arg<options::TableCreateOptions>)
);

create_cmd!(
    /// Drop a table from a database. The table and all its data will be deleted.
    ///
    /// If successful, the command returns an object with two fields:
    ///
    /// - `tables_dropped`: always `1`.
    /// - `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    ///   - `old_val`: the dropped table’s [config](Command::config) value.
    ///   - `new_val`: always `null`.
    ///
    /// If the given table does not exist in the database, the command throws ReqlRuntimeError.
    ///
    /// ## Example
    /// Drop a table named 'dc_universe'.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.db("test").table_drop("dc_universe").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [table_create](Self::table_create)
    /// - [table_list](Self::table_list)
    table_drop(table_name: Serialize)
);

create_cmd!(
    /// List all the secondary indexes of this table.
    ///
    /// ## Example
    /// List the available secondary indexes for this table.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").index_list().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [index_create](Self::index_create)
    /// - [index_drop](Self::index_drop)
    only_command,
    index_list,
);

create_cmd!(
    /// Create a new secondary index on a table.
    ///
    /// Secondary indexes improve the speed of many read queries at the slight cost of increased storage space
    /// and decreased write performance. For more information about secondary indexes,
    /// read the article "[Using secondary indexes in RethinkDB](https://rethinkdb.com/docs/secondary-indexes/javascript/)."
    ///
    /// RethinkDB supports different types of secondary indexes:
    ///
    /// - *Simple indexes* based on the value of a single field.
    /// - *Compound indexes* based on multiple fields.
    /// - *Multi indexes* based on arrays of values, created when the multi optional argument is true.
    /// - *Geospatial indexes* based on indexes of geometry objects, created when the geo optional argument is true.
    /// - Indexes based on *arbitrary expressions*.
    ///
    /// The `indexFunction` can be an anonymous function or a binary representation
    /// obtained from the function field of [`index_status`](Command::index_status).
    /// The function must be deterministic, and so cannot use a subquery or the r.js command.
    ///
    /// If successful, `create_index` will return an object of the form `{"created": 1}`.
    /// If an index by that name already exists on the table, a `ReqlRuntimeError` will be thrown.
    ///
    /// *Note* that an index may not be immediately available after creation.
    /// If your application needs to use indexes immediately after creation,
    /// use the [`index_wait`](Command::index_wait) command to ensure the indexes are ready before use.
    ///
    /// ## Example
    /// Create a simple index based on the field `postId`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("comments").index_create("postId").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a geospatial index based on the field `location`.
    ///
    /// ```
    /// # use unreql::cmd::options::IndexCreateOptions;
    /// # unreql::example(|r, conn| {
    /// let opts = IndexCreateOptions::new().geo(true);
    /// r.table("comments").index_create(r.with_opt("location", opts)).run(conn)
    /// # })
    /// ```
    ///
    /// A geospatial index field should contain only geometry objects.
    /// It will work with geometry ReQL terms ([get_intersecting](Command::get_intersecting) and [get_nearest](Command::get_nearest))
    /// as well as index-specific terms ([index_status](Command::index_status), [index_wait](Command::index_wait), [index_drop](Command::index_drop) and [index_list](Command::index_list)).
    /// Using terms that rely on non-geometric ordering such as getAll, orderBy and between will result in an error.
    ///
    /// ## Example
    /// Create a simple index based on the nested field `author > name`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("comments")
    ///   .index_create(r.args(("authorName", r.row().g("author").g("name"))))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a compound index based on the fields `postId` and `date`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("comments")
    ///   .index_create(r.args(("postAndDate", [r.row().g("postId"), r.row().g("date")])))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a multi index based on the field `authors`.
    ///
    /// ```
    /// # use unreql::cmd::options::IndexCreateOptions;
    /// # unreql::example(|r, conn| {
    /// let opts = IndexCreateOptions::new().multi(true);
    /// r.table("posts").index_create(r.with_opt("authors", opts)).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a geospatial multi index based on the field towers.
    ///
    /// ```
    /// # use unreql::cmd::options::IndexCreateOptions;
    /// # unreql::example(|r, conn| {
    /// let opts = IndexCreateOptions::new()
    ///   .multi(true)
    ///   .geo(true);
    /// r.table("networks").index_create(r.with_opt("towers", opts)).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create an index based on an arbitrary expression.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .index_create(r.args(("authors", func!(|doc| {
    ///     r.branch(
    ///       doc.clone().has_fields("updatedAt"),
    ///       doc.clone().g("updatedAt"),
    ///       doc.g("createdAt"),
    ///     )
    ///   }))))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Rebuild an outdated secondary index on a table.
    ///
    /// ```
    /// # use unreql::{func, cmd::options::IndexRenameOptions};
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .index_status("oldIndex")
    ///   .nth(0)
    ///   .do_(func!(|oldIndex| {
    ///     r.table("posts").index_create(r.args(("newIndex", oldIndex.g("function")))).do_(func!(|| {
    ///       r.table("posts").index_wait("newIndex").do_(func!(|| {
    ///         let opts = IndexRenameOptions::new().overwrite(true);
    ///         r.table("posts").index_rename("newIndex", "oldIndex", opts)
    ///       }))
    ///     }))
    ///   }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [index_wait](Self::index_wait)
    /// - [index_status](Self::index_status)
    /// - [index_list](Self::index_list)
    /// - [index_drop](Self::index_drop)
    only_command,
    index_create(index: ManyArgs<options::IndexCreateOptions>)
);

create_cmd!(
    /// Delete a previously created secondary index of this table.
    ///
    /// ## Example
    /// Drop a secondary index named 'code_name'.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("dc").index_drop("code_name").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [index_create](Self::index_create)
    /// - [index_list](Self::index_list)
    only_command,
    index_drop(index_name: Serialize)
);

create_cmd!(
    /// Rename an existing secondary index on a table.
    ///
    /// If the optional argument `overwrite` is specified as `true`,
    /// a previously existing index with the new name will be deleted and the index will be renamed.
    /// If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.
    ///
    /// The return value on success will be an object of the format `{renamed: 1}`, or `{renamed: 0}` if the old and new names are the same.
    ///
    /// An error will be raised if the old index name does not exist, if the new index name
    /// is already in use and `overwrite` is `false`, or if either the old or new index name are the same as the primary key field name.
    ///
    /// ## Example
    /// Rename an index on the comments table.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("comments").index_rename("postId", "messageId", ()).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [index_create](Self::index_create)
    /// - [index_status](Self::index_status)
    /// - [index_list](Self::index_list)
    /// - [index_drop](Self::index_drop)
    only_command,
    index_rename(old_index_name: Serialize, new_index_name: Serialize, opt: Opt<options::IndexRenameOptions>)
);

create_cmd!(
    /// Get the status of the specified indexes on this table, or the status of all indexes on this table if no indexes are specified.
    ///
    /// The result is an array where for each index, there will be an object like this one:
    ///
    /// ```text
    /// {
    ///     index: <indexName>,
    ///     ready: true,
    ///     function: <binary>,
    ///     multi: <bool>,
    ///     geo: <bool>,
    ///     outdated: <bool>
    /// }
    /// ```
    ///
    /// or this one:
    ///
    /// ```text
    /// {
    ///     index: <indexName>,
    ///     ready: false,
    ///     progress: <float>,
    ///     function: <binary>,
    ///     multi: <bool>,
    ///     geo: <bool>,
    ///     outdated: <bool>
    /// }
    /// ```
    ///
    /// The `multi` field will be `true` or `false` depending on whether this index was created as a multi index;
    /// the `geo` field will be `true` or `false` depending on whether this index was created as a geospatial index.
    /// See [index_create](Command::index_create) for details. The `outdated` field will be true if the index is outdated
    /// in the current version of RethinkDB and needs to be rebuilt. The `progress` field is a float between `0` and `1`,
    /// indicating how far along the server is in constructing indexes after the most recent change to the table
    /// that would affect them. (`0` indicates no such indexes have been constructed; `1` indicates all of them have.)
    ///
    /// The `function` field is a binary object containing an opaque representation of the secondary index
    /// (including the `multi` argument if specified). It can be passed as the second argument to `index_create`
    /// to create a new index with the same function; see [index_create] for more information.
    ///
    /// ## Example
    /// Get the status of all the indexes on test:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("test").index_status(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get the status of the timestamp index:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("test").index_status("timestamp").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [index_wait](Self::index_wait)
    only_command,
    index_status(index: ManyArgs<()>)
);

create_cmd!(
    /// Wait for the specified indexes on this table to be ready, or for all indexes on this table to be ready if no indexes are specified.
    ///
    /// The result is an array containing one object for each table index:
    ///
    /// ```text
    /// {
    ///     index: <indexName>,
    ///     ready: true,
    ///     function: <binary>,
    ///     multi: <bool>,
    ///     geo: <bool>,
    ///     outdated: <bool>
    /// }
    /// ```
    ///
    /// See the indexStatus documentation for a description of the field values.
    ///
    /// ## Example
    /// Wait for all indexes on the table test to be ready:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("test").index_wait(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Wait for the index timestamp to be ready:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("test").index_wait("timestamp").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [index_status](Self::index_status)
    only_command,
    index_wait(index: ManyArgs<()>)
);
