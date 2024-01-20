use ql2::term::TermType;
use unreql_macros::create_cmd;

use crate::{
    cmd::{
        args::{Arg, ManyArgs, Opt},
        options,
    },
    Command,
};

create_cmd!(
    /// Insert documents into a table. Accepts a single document or an array of documents.
    ///
    /// If returnChanges is set to true or "always", the changes array will follow the same order
    /// as the inserted documents. Documents in changes for which an error occurs (such as a key conflict)
    /// will have a third field, error, with an explanation of the error.
    ///
    /// Insert returns an object that contains the following attributes:
    /// - `inserted`: the number of documents successfully inserted.
    /// - `replaced`: the number of documents updated when `conflict` is set to `"replace"` or `"update"`.
    /// - `unchanged`: the number of documents whose fields are identical to existing documents
    ///   with the same primary key when `conflict` is set to `"replace"` or `"update"`.
    /// - `errors`: the number of errors encountered while performing the insert.
    /// - `first_error`: If errors were encountered, contains the text of the first error.
    /// - `deleted` and `skipped`: 0 for an insert operation.
    /// - `generated_keys`: a list of generated primary keys for inserted documents whose
    ///   primary keys were not specified (capped to 100,000).
    /// - `warnings`: if the field `generated_keys` is truncated, you will get the warning
    ///   *“Too many generated keys (<X>), array truncated to 100000.”*.
    /// - `changes`: if `returnChanges` is set to `true`, this will be an array of objects,
    ///   one for each objected affected by the `insert` operation. Each object will have two keys:
    ///   `{new_val: <new value>, old_val: null}`.
    ///
    /// <div class="warning">
    /// RethinkDB write operations will only throw exceptions if errors occur before any writes.
    /// Other errors will be listed in first_error, and errors will be set to a non-zero count.
    /// To properly handle errors with this term, code must both handle exceptions
    /// and check the errors return value!
    /// </div>
    ///
    /// ## Example
    /// Insert a document into the table `posts`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").insert(rjson!({
    ///   "id": 1,
    ///   "title": "Lorem ipsum",
    ///   "content": "Dolor sit amet",
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// The result will be:
    ///
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors": 0,
    ///     "inserted": 1,
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    ///
    /// ## Example
    /// Insert a document without a defined primary key into the table `posts` where the primary key is `id`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").insert(rjson!({
    ///   "title": "Lorem ipsum",
    ///   "content": "Dolor sit amet",
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// RethinkDB will generate a primary key and return it in `generated_keys`.
    ///
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors": 0,
    ///     "generated_keys": [
    ///         "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ///     ],
    ///     "inserted": 1,
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    ///
    /// Retrieve the document you just inserted with:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get("dd782b64-70a7-43e4-b65e-dd14ae61d947").run(conn)
    /// # })
    /// ```
    ///
    /// And you will get back:
    ///
    /// ```json
    /// {
    ///     "id": "dd782b64-70a7-43e4-b65e-dd14ae61d947",
    ///     "title": "Lorem ipsum",
    ///     "content": "Dolor sit amet"
    /// }
    /// ```
    ///
    /// ## Example
    /// Insert multiple documents into the table `users`.
    ///
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").insert(r.args([
    ///     rjson!({"id": "william", "email": "william@rethinkdb.com"}),
    ///     rjson!({"id": "lara", "email": "lara@rethinkdb.com"}),
    /// ])).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Insert a document into the table `users`, replacing the document if it already exists.
    ///
    /// ```
    /// # use unreql::cmd::options::{InsertOptions, Conflict};
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").insert(r.with_opt(
    ///     rjson!({"id": "william", "email": "william@rethinkdb.com"}),
    ///     InsertOptions{ conflict: Some(Conflict::Replace), ..Default::default() },
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Copy the documents from posts to postsBackup.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("postsBackup").insert(r.table("posts")).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get back a copy of the inserted document (with its generated primary key).
    ///
    /// ```
    /// # use unreql::cmd::options::InsertOptions;
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").insert(r.with_opt(
    ///     rjson!({"title": "Lorem ipsum", "content": "Dolor sit amet"}),
    ///     InsertOptions {return_changes: Some(true.into()), ..Default::default() },
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// The result will be
    ///
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors": 0,
    ///     "generated_keys": [
    ///         "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ///     ],
    ///     "inserted": 1,
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0,
    ///     "changes": [
    ///         {
    ///             "old_val": null,
    ///             "new_val": {
    ///                 "id": "dd782b64-70a7-43e4-b65e-dd14ae61d947",
    ///                 "title": "Lorem ipsum",
    ///                 "content": "Dolor sit amet"
    ///             }
    ///         }
    ///     ]
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [update](Self::update)
    /// - [replace](Self::replace)
    /// - [delete](Self::delete)
    only_command,
    insert(object: ManyArgs<options::InsertOptions>)
);

create_cmd!(
    /// Update JSON documents in a table. Accepts a JSON document, a ReQL expression, or a combination of the two.
    ///
    /// Update returns an object that contains the following attributes:
    ///
    /// - `replaced`: the number of documents that were updated.
    /// - `unchanged`: the number of documents that would have been modified except
    ///   the new value was the same as the old value.
    /// - `skipped`: the number of documents that were skipped because the document didn’t exist.
    /// - `errors`: the number of errors encountered while performing the update.
    /// - `first_error`: If errors were encountered, contains the text of the first error.
    /// - `deleted` and `inserted`: 0 for an update operation.
    /// - `changes`: if `returnChanges` is set to `true`, this will be an array of objects,
    ///   one for each objected affected by the `update` operation.
    ///   Each object will have two keys: `{new_val: <new value>, old_val: <old value>}`.
    ///
    /// <div class="warning">
    /// RethinkDB write operations will only throw exceptions if errors occur before any writes.
    /// Other errors will be listed in first_error, and errors will be set to a non-zero count.
    /// To properly handle errors with this term, code must both handle exceptions
    /// and check the errors return value!
    /// </div>
    ///
    /// ## Example
    /// Update the status of the post with `id` of `1` to `published`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(rjson!({"status": "published"})).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Update the status of all posts to `published`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").update(rjson!({"status": "published"})).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Update the status of all the posts written by William.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .filter(rjson!({"author": "William"}))
    ///   .update(rjson!({"status": "published"}))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// <div class="warning">
    /// Note that filter, getAll and similar operations do not execute in an atomic fashion
    /// with update. Read Consistency guarantees for more details. Also, see the example for
    /// conditional updates below for a solution using branch in an update clause.
    /// </div>
    ///
    /// ## Example
    /// Increment the field `view` of the post with `id` of `1`. This query will throw an error if the field `views` doesn’t exist.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(rjson!({
    ///     "views": r.row().g("views").add(1),
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Increment the field `view` of the post with `id` of `1`. If the field `views` does not exist, it will be set to `0`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(rjson!({
    ///     "views": r.row().g("views").add(1).default(0),
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Perform a conditional update.
    /// If the post has more than 100 views, set the `type` of a post to `hot`, else set it to `normal`.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(func!(|post| {
    ///     r.branch(
    ///         post.g("views").gt(100),
    ///         rjson!({ "type": "hot" }),
    ///         rjson!({ "type": "normal" }),
    ///     )
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Update the field `numComments` with the result of a sub-query.
    /// Because this update is not atomic, you must pass the `non_atomic` flag.
    ///
    /// ```
    /// # use unreql::cmd::options::UpdateOptions;
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(r.with_opt(
    ///     rjson!({ "numComments": r.table("comments").filter(rjson!({ "idPost": 1 })).count(()) }),
    ///     UpdateOptions { non_atomic: Some(true), ..Default::default() },
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// If you forget to specify the `non_atomic` flag, you will get a `ReqlRuntimeError`:
    ///
    /// ```text
    /// ReqlRuntimeError: Could not prove function deterministic.  Maybe you want to use the non_atomic flag?
    /// ```
    ///
    /// ## Example
    /// Update the field `numComments` with a random value between 0 and 100.
    /// This update cannot be proven deterministic because of `r.js` (and in fact is not),
    /// so you must pass the `non_atomic` flag.
    ///
    /// ```
    /// # use unreql::cmd::options::UpdateOptions;
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(r.with_opt(
    ///     rjson!({ "numComments": r.js("Math.floor(Math.random()*100)") }),
    ///     UpdateOptions { non_atomic: Some(true), ..Default::default() },
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Update the status of the post with `id` of `1` using soft durability.
    ///
    /// ```
    /// # use unreql::cmd::options::{UpdateOptions, Durability};
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(r.with_opt(
    ///     rjson!({ "status": "published" }),
    ///     UpdateOptions { durability: Some(Durability::Soft), ..Default::default() },
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Increment the field `views` and return the values of the document before
    /// and after the update operation.
    ///
    /// ```
    /// # use unreql::cmd::options::UpdateOptions;
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(r.with_opt(
    ///     rjson!({ "views": r.row().g("views").add(1) }),
    ///     UpdateOptions { return_changes: Some(true.into()), ..Default::default() },
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// The result will now include a changes field:
    ///
    /// ```json
    /// {
    ///     "deleted": 1,
    ///     "errors": 0,
    ///     "inserted": 0,
    ///     "changes": [
    ///         {
    ///             "new_val": {
    ///                 "id": 1,
    ///                 "author": "Julius_Caesar",
    ///                 "title": "Commentarii de Bello Gallico",
    ///                 "content": "Aleas jacta est",
    ///                 "views": 207
    ///             },
    ///             "old_val": {
    ///                 "id": 1,
    ///                 "author": "Julius_Caesar",
    ///                 "title": "Commentarii de Bello Gallico",
    ///                 "content": "Aleas jacta est",
    ///                 "views": 206
    ///             }
    ///         }
    ///     ],
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    ///
    /// # Updating nested fields
    ///
    /// The update command supports RethinkDB’s nested field syntax to update subdocuments.
    /// Consider a user table with contact information in this format:
    ///
    /// ```json
    /// {
    /// 	"id": 10001,
    /// 	"name": "Bob Smith",
    /// 	"contact": {
    /// 		"phone": {
    /// 			"work": "408-555-1212",
    /// 			"home": "408-555-1213",
    /// 			"cell": "408-555-1214"
    /// 		},
    /// 		"email": {
    /// 			"work": "bob@smith.com",
    /// 			"home": "bobsmith@example.com",
    /// 			"other": "bobbys@moosecall.net"
    /// 		},
    /// 		"im": {
    /// 			"skype": "Bob Smith",
    /// 			"aim": "bobmoose",
    /// 			"icq": "nobodyremembersicqnumbers"
    /// 		}
    /// 	},
    /// 	"notes": [
    /// 		{
    /// 			"date": r.time(2014,1,1,'Z'),
    /// 			"from": "John Doe",
    /// 			"subject": "My name is even more boring than Bob's"
    /// 		},
    /// 		{
    /// 			"date": r.time(2014,2,2,'Z'),
    /// 			"from": "Bob Smith Sr",
    /// 			"subject": "Happy Second of February"
    /// 		}
    /// 	]
    /// }
    /// ```
    ///
    /// ## Example
    /// Update Bob Smith’s cell phone number.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).update(rjson!(
    ///     { "contact": { "phone": { "cell": "408-555-4242" }}}
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Add another note to Bob Smith’s record.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// let new_note = rjson!({
    ///     "date": r.now(),
    ///     "from": "Inigo Montoya",
    ///     "subject": "You killed my father",
    /// });
    /// r.table("posts").get(1).update(rjson!(
    ///     { "notes": r.row().g("notes").append(new_note) }
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// This will fail if the `notes` field does not exist in the document.
    /// To perform this as an “upsert” (update or insert),
    /// use the [default](Command::default) command to ensure the field is initialized as an empty list.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// # let new_note = rjson!({});
    /// r.table("posts").get(1).update(rjson!(
    ///     { "notes": r.row().g("notes").default(rjson!([])).append(new_note) }
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example: Send a note to every user with an ICQ number.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// let icq_note = rjson!({
    ///    "date": r.now(),
    ///    "from": "Admin",
    ///    "subject": "Welcome to the future",
    /// });
    /// r.table("posts")
    ///    .filter(r.row().has_fields(rjson!({ "contact": { "im": "icq" }})))
    ///    .update(rjson!({ "notes": r.row().g("notes").append(icq_note) }))
    ///    .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Replace all of Bob’s IM records.
    /// Normally, `update` will merge nested documents together;
    /// to replace the entire `"im"` document, use the [literal](r::literal) command.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").get(10001).update(rjson!(
    ///     { "contact": { "im": r.literal(rjson!({ "aim": "themoosemeister" }))}}
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [insert](Self::insert)
    /// - [replace](Self::replace)
    /// - [delete](Self::delete)
    only_command,
    update(object: Arg<options::UpdateOptions>)
);

create_cmd!(
    /// Replace documents in a table.
    ///
    /// Accepts a JSON document or a ReQL expression, and replaces the original document
    /// with the new one. The new document must have the same primary key as the original
    /// document.
    ///
    /// The replace command can be used to both insert and delete documents.
    /// If the “replaced” document has a primary key that doesn’t exist in the table,
    /// the document will be inserted; if an existing document is replaced with null,
    /// the document will be deleted. Since update and replace operations are performed
    /// atomically, this allows atomic inserts and deletes as well.
    ///
    /// Replace returns an object that contains the following attributes:
    ///
    /// - `replaced`: the number of documents that were replaced.
    /// - `unchanged`: the number of documents that would have been modified,
    ///   except that the new value was the same as the old value.
    /// - `inserted`: the number of new documents added. A document is considered inserted
    ///   if its primary key did not exist in the table at the time of the `replace` operation.
    /// - `deleted`: the number of deleted documents when doing a replace with `null`.
    /// - `errors`: the number of errors encountered while performing the replace.
    /// - `first_error`: If errors were encountered, contains the text of the first error.
    /// - `skipped`: 0 for a replace operation.
    /// - `changes`: if `returnChanges` is set to `true`, this will be an array of objects,
    ///   one for each objected affected by the `replace` operation.
    ///   Each object will have two keys: `{new_val: <new value>, old_val: <old value>}`.
    ///
    /// <div class="warning">
    /// RethinkDB write operations will only throw exceptions if errors occur before any
    /// writes. Other errors will be listed in `first_error`, and `errors` will be set
    /// to a non-zero count. To properly handle errors with this term, code must both
    /// handle exceptions and check the `errors` return value!
    /// </div>
    ///
    /// ## Example
    /// Replace the document with the primary key `1`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).replace(rjson!({
    ///     "id": 1,
    ///     "title": "Lorem ipsum",
    ///     "content": "Aleas jacta est",
    ///     "status": "draft"
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Remove the field `status` from all posts.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").replace(func!(|post| {
    ///     post.without("status")
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Remove all the fields that are not `id`, `title` or `content`.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").replace(func!(|post| {
    ///     post.pluck(r.args(["id", "title", "content"]))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Replace the document with the primary key `1` using soft durability.
    ///
    /// ```
    /// # use unreql::cmd::options::ReplaceOptions;
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get(1).replace(r.with_opt(
    ///     rjson!({
    ///         "id": 1,
    ///         "title": "Lorem ipsum",
    ///         "content": "Aleas jacta est",
    ///         "status": "published"
    ///     }),
    ///     ReplaceOptions { return_changes: Some(true.into()), ..Default::default() }
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// The result will have two fields `old_val` and `new_val`.
    ///
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors":  0,
    ///     "inserted": 0,
    ///     "changes": [
    ///         {
    ///             "new_val": {
    ///                 "id":1,
    ///                 "title": "Lorem ipsum"
    ///                 "content": "Aleas jacta est",
    ///                 "status": "published",
    ///             },
    ///             "old_val": {
    ///                 "id":1,
    ///                 "title": "Lorem ipsum"
    ///                 "content": "TODO",
    ///                 "status": "draft",
    ///                 "author": "William",
    ///             }
    ///         }
    ///     ],
    ///     "replaced": 1,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [insert](Self::insert)
    /// - [update](Self::update)
    /// - [delete](Self::delete)
    only_command,
    replace(object: Arg<options::ReplaceOptions>)
);

create_cmd!(
    /// Delete one or more documents from a table.
    ///
    /// Delete returns an object that contains the following attributes:
    ///
    /// - `deleted`: the number of documents that were deleted.
    /// - `skipped`: the number of documents that were skipped.
    ///   For example, if you attempt to delete a batch of documents, and another concurrent query deletes some of those documents first, they will be counted as skipped.
    /// - `errors`: the number of errors encountered while performing the delete.
    /// - `first_error`: If errors were encountered, contains the text of the first error.
    /// - `inserted`, `replaced`, and `unchanged`: all 0 for a delete operation..
    /// - `changes`: if `returnChanges` is set to `true`, this will be an array of objects,
    ///   one for each objected affected by the `delete` operation. Each object will
    ///   have two keys: `{new_val: null, old_val: <old value>}`.
    ///
    /// <div class="warning">
    /// RethinkDB write operations will only throw exceptions if errors occur before
    /// any writes. Other errors will be listed in first_error, and errors will be set
    /// to a non-zero count. To properly handle errors with this term, code must both
    /// handle exceptions and check the errors return value!
    /// </div>
    ///
    /// ## Example
    /// Delete a single document from the table `comments`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("comments")
    ///   .get("7eab9e63-73f1-4f33-8ce4-95cbea626f59")
    ///   .delete(())
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Delete all documents from the table `comments`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("comments").delete(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Delete all comments where the field `idPost` is `3`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("comments").filter(rjson!({ "idPost": 3 })).delete(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Delete a single document from the table `comments` and return its value.
    ///
    /// ```
    /// # use unreql::cmd::options::DeleteOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("comments")
    ///   .get("7eab9e63-73f1-4f33-8ce4-95cbea626f59")
    ///   .delete(DeleteOptions {
    ///     return_changes: Some(true.into()),
    ///     ..Default::default()
    ///   })
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// The result will look like:
    ///
    /// ```json
    /// {
    ///     "deleted": 1,
    ///     "errors": 0,
    ///     "inserted": 0,
    ///     "changes": [
    ///         {
    ///             "new_val": null,
    ///             "old_val": {
    ///                 "id": "7eab9e63-73f1-4f33-8ce4-95cbea626f59",
    ///                 "author": "William",
    ///                 "comment": "Great post",
    ///                 "idPost": 3
    ///             }
    ///         }
    ///     ],
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    ///
    /// ## Example
    /// Delete all documents from the table `comments` without waiting for the operation
    /// to be flushed to disk.
    ///
    /// ```
    /// # use unreql::cmd::options::{DeleteOptions, Durability};
    /// # unreql::example(|r, conn| {
    /// r.table("comments")
    ///   .delete(DeleteOptions {
    ///     durability: Some(Durability::Soft),
    ///     ..Default::default()
    ///   })
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [insert](Self::insert)
    /// - [update](Self::update)
    /// - [replace](Self::replace)
    only_command,
    delete(opt: Opt<options::DeleteOptions>)
);

create_cmd!(
    /// `sync` ensures that writes on a given table are written to permanent storage.
    ///
    /// Queries that specify soft durability ({durability: 'soft'}) do not give such
    /// guarantees, so sync can be used to ensure the state of these queries.
    /// A call to sync does not return until all previous writes to the table are persisted.
    ///
    /// If successful, the operation returns an object: {synced: 1}.
    ///
    /// ## Example
    /// After having updated multiple heroes with soft durability, we now want to wait until these changes are persisted.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").sync().run(conn)
    /// # });
    /// ```
    ///
    /// # Related commands
    /// - [noreply_wait](Session::noreply_wait)
    only_command,
    sync,
);
