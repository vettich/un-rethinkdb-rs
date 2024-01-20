use unreql_macros::create_cmd;
use ql2::term::TermType;
use serde::Serialize;

use crate::Command;

create_cmd!(
    /// List all database names in the system. The result is a list of strings.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.db_list().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [db_create](Self::db_create)
    /// - [db_drop](Self::db_drop)
    /// - [table_create](Self::table_create)
    only_root,
    db_list,
);

create_cmd!(
    /// Create a database. A RethinkDB database is a collection of tables, similar to relational databases.
    ///
    /// If successful, the command returns an object with two fields:
    ///
    /// - `dbs_created`: always `1`.
    /// - `config_changes`: a list containing one object with two fields, `old_val` and `new_val`:
    ///   - `old_val`: always `null`.
    ///   - `new_val`: the database’s new [config](Command::config) value.
    ///
    /// If a database with the same name already exists, the command throws ReqlRuntimeError.
    ///
    /// *Note*: Only alphanumeric characters, hyphens and underscores are valid for the database name.
    ///
    /// ## Example
    /// Create a database named 'superheroes'.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.db_create("superheroes").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [db_drop](Self::db_drop)
    /// - [db_list](Self::db_list)
    /// - [table_create](Self::table_create)
    only_root,
    db_create(db_name: Serialize)
);

create_cmd!(
    /// Drop a database. The database, all its tables, and corresponding data will be deleted.
    ///
    /// If successful, the command returns an object with two fields:
    ///
    /// - `dbs_dropped`: always `1`.
    /// - `tables_dropped`: the number of tables in the dropped database.
    /// - `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    ///   - `old_val`: the database’s original [config](Command::config) value.
    ///   - `new_val`: always null.
    ///
    /// If the given database does not exist, the command throws ReqlRuntimeError.
    ///
    /// ## Example
    /// Drop a database named 'superheroes'.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.db_drop("superheroes").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [db_create](Self::db_create)
    /// - [db_list](Self::db_list)
    /// - [table_create](Self::table_create)
    only_root,
    db_drop(db_name: Serialize)
);
