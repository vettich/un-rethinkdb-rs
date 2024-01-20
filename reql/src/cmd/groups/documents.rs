use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::args::{ManyArgs, OneAndSecondOptionalArg},
    Command,
};

create_cmd!(
    /// Returns the currently visited document.
    ///
    /// *Note* that `row` does not work within subqueries to access nested documents;
    /// you should use anonymous functions to access those documents instead.
    /// (See the last example.)
    ///
    /// ## Example
    /// Get all users whose age is greater than 5.
    ///
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.row().g("age").gt(5)).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Access the attribute ‘child’ of an embedded document.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.row().g("embedded_doc").g("child").gt(5)).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Add 1 to every element of an array.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([1, 2, 3]).map(r.row().add(1)).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// For nested queries, use functions instead of `row`.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|doc| {
    ///   doc.g("name").eq(r.table("prizes").get("winner"))
    /// })).run(conn)
    /// # })
    /// ```
    only_root,
    row:ImplicitVar
);

create_cmd!(
    /// Plucks out one or more attributes from either an object or a sequence of
    /// objects (projection).
    ///
    /// ## Example
    /// We just need information about IronMan’s reactor and not the rest of
    /// the document.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").pluck(r.args(["reactorState", "reactorPower"])).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// For the hero beauty contest we only care about certain qualities.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").pluck(r.args(["beauty", "muscleTone", "charm"])).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Pluck can also be used on nested objects.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .pluck(rjson!({"abilities" : {"damage" : true, "mana_cost" : true}, "weapons" : true}))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// The nested syntax can quickly become overly verbose so there’s a shorthand for it.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .pluck(r.args((rjson!({"abilities" : ["damage", "mana_cost"]}), "weapons")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// For more information read the [nested field documentation](https://rethinkdb.com/docs/nested-fields/javascript/).
    ///
    /// # Related commands
    /// - [without](Self::without)
    /// - [map](Self::map)
    only_command,
    pluck(selector: ManyArgs<()>)
);

create_cmd!(
    /// The opposite of pluck; takes an object or a sequence of objects, and
    /// returns them with the specified paths removed.
    ///
    /// ## Example
    /// Since we don’t need it for this computation we’ll save bandwidth and
    /// leave out the list of IronMan’s romantic conquests.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").without("personalVictoriesList").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Without their prized weapons, our enemies will quickly be vanquished.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("enemies").without("weapons").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Nested objects can be used to remove the damage subfield from the weapons
    /// and abilities fields.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .without(rjson!({"weapons" : {"damage" : true}, "abilities" : {"damage" : true}}))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// The nested syntax can quickly become overly verbose so there’s a shorthand
    /// for it.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .without(rjson!({"weapons":"damage", "abilities":"damage"}))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [pluck](Self::pluck)
    /// - [map](Self::map)
    only_command,
    without(selector: ManyArgs<()>)
);

create_cmd!(
    /// Merge two or more objects together to construct a new object with
    /// properties from all.
    ///
    /// When there is a conflict between field names, preference is given to
    /// fields in the rightmost object in the argument list. `merge` also accepts
    /// a subquery function that returns an object, which will be used similarly
    /// to a map function.
    ///
    /// ## Example
    /// Equip Thor for battle.
    ///
    /// ```
    /// # use unreql::r;
    /// r.table("marvel").get("thor").merge(
    ///   r.args([
    ///     r.table("equipment").get("hammer"),
    ///     r.table("equipment").get("pimento_sandwich")
    ///   ])
    /// );
    /// ```
    ///
    /// ## Example
    /// Equip every hero for battle, using a subquery function to retrieve their weapons.
    ///
    /// ```
    /// # use unreql::{r, func};
    /// # use unreql::rjson;
    /// r.table("marvel").merge(func!(|hero| {
    ///   r.expr(rjson!({
    ///     "weapons": r.table("weapons").get(hero.g("weaponId")),
    ///   }))
    /// }));
    /// ```
    ///
    /// ## Example
    /// Use `merge` to join each blog post with its comments.
    ///
    /// *Note* that the sequence being merged—in this example, the comments—must
    /// be coerced from a selection to an array. Without `coerce_to` the operation
    /// will throw an error (“Expected type DATUM but found SELECTION”).
    ///
    /// ```
    /// # use unreql::{r, func};
    /// # use unreql::rjson;
    /// r.table("posts").merge(func!(|post| {
    ///   r.expr(rjson!({
    ///     "comments": r.table("comments")
    ///       .get_all(r.with_opt(post.g("id"), r.index("postId")))
    ///       .coerce_to("array")
    ///   }))
    /// }));
    /// ```
    ///
    /// ## Example
    /// Merge can be used recursively to modify object within objects.
    ///
    /// ```
    /// # use unreql::{r, func};
    /// # use unreql::rjson;
    /// r.expr(rjson!({"weapons" : {"spectacular_graviton_beam" : {"dmg" : 10, "cooldown" : 20}}}))
    ///   .merge(rjson!({"weapons" : {"spectacular_graviton_beam" : {"dmg" : 10}}}));
    /// ```
    ///
    /// ## Example
    /// To replace a nested object with another object you can use the literal keyword.
    ///
    /// ```
    /// # use unreql::{r, func};
    /// # use unreql::rjson;
    /// r.expr(rjson!({"weapons" : {"spectacular_graviton_beam" : {"dmg" : 10, "cooldown" : 20}}}))
    ///   .merge(rjson!({"weapons" : r.literal(rjson!({"repulsor_rays" : {"dmg" : 3, "cooldown" : 0}}))}));
    /// ```
    ///
    /// ## Example
    /// Literal can be used to remove keys from an object as well.
    ///
    /// ```
    /// # use unreql::{r, func};
    /// # use unreql::rjson;
    /// r.expr(rjson!({"weapons" : {"spectacular_graviton_beam" : {"dmg" : 10, "cooldown" : 20}}}))
    ///   .merge(rjson!({"weapons" : {"spectacular_graviton_beam" : r.literal(())}}));
    /// ```
    ///
    /// # Related commands
    /// - [pluck](Self::pluck)
    /// - [without](Self::without)
    /// - [map](Self::map)
    only_command,
    merge(objects_or_functions: ManyArgs<()>)
);

create_cmd!(
    /// Append a value to an array.
    ///
    /// ## Example
    /// Retrieve Iron Man’s equipment list with the addition of some new boots.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("equipment").append("newBoots").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [prepend](Self::prepend)
    /// - [insert_at](Self::insert_at)
    /// - [delete_at](Self::delete_at)
    /// - [change_at](Self::change_at)
    /// - [merge](Self::merge)
    append(value: Serialize)
);

create_cmd!(
    /// Prepend a value to an array.
    ///
    /// ## Example
    /// Retrieve Iron Man’s equipment list with the addition of some new boots.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("equipment").prepend("newBoots").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [append](Self::append)
    /// - [insert_at](Self::insert_at)
    /// - [delete_at](Self::delete_at)
    /// - [change_at](Self::change_at)
    /// - [merge](Self::merge)
    prepend(value: Serialize)
);

create_cmd!(
    /// Remove the elements of one array from another array.
    ///
    /// ## Example
    /// Retrieve Iron Man’s equipment list without boots.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("equipment").difference(["Boots"]).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Remove Iron Man’s boots from his equipment.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .get("IronMan")
    ///   .update(rjson!({ "equipment": r.row().g("equipment").difference(["Boots"]) }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [set_insert](Self::set_insert)
    /// - [set_union](Self::set_union)
    /// - [set_intersection](Self::set_intersection)
    /// - [set_difference](Self::set_difference)
    /// - [union](Self::union)
    difference(array: Serialize)
);

create_cmd!(
    /// Add a value to an array and return it as a set (an array with distinct values).
    ///
    /// ## Example
    /// Retrieve Iron Man’s equipment list with the addition of some new boots.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("equipment").set_insert("newBoots").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [difference](Self::difference)
    /// - [set_union](Self::set_union)
    /// - [set_intersection](Self::set_intersection)
    /// - [set_difference](Self::set_difference)
    /// - [union](Self::union)
    set_insert(value: Serialize)
);

create_cmd!(
    /// Add a several values to an array and return it as a set (an array with distinct values).
    ///
    /// # Example
    /// Retrieve Iron Man’s equipment list with the addition of some new boots and an arc reactor.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("equipment").set_union(["newBoots", "arc_reactor"]).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [difference](Self::difference)
    /// - [set_insert](Self::set_insert)
    /// - [set_intersection](Self::set_intersection)
    /// - [set_difference](Self::set_difference)
    /// - [union](Self::union)
    set_union(array: Serialize)
);

create_cmd!(
    /// Intersect two arrays returning values that occur in both of them as a set (an array with distinct values).
    ///
    /// ## Example
    /// Check which pieces of equipment Iron Man has from a fixed list.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("equipment").set_intersection(["newBoots", "arc_reactor"]).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [difference](Self::difference)
    /// - [set_insert](Self::set_insert)
    /// - [set_union](Self::set_union)
    /// - [set_difference](Self::set_difference)
    /// - [union](Self::union)
    set_intersection(array: Serialize)
);

create_cmd!(
    /// Remove the elements of one array from another and return them as a set (an array with distinct values).
    ///
    /// ## Example
    /// Check which pieces of equipment Iron Man has, excluding a fixed list.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("equipment").set_difference(["newBoots", "arc_reactor"]).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [difference](Self::difference)
    /// - [set_insert](Self::set_insert)
    /// - [set_union](Self::set_union)
    /// - [set_intersection](Self::set_intersection)
    /// - [union](Self::union)
    set_difference(array: Serialize)
);

create_cmd!(
    /// Get a single field from an object.
    ///
    /// If called on a sequence, gets that field from every object in the
    /// sequence, skipping objects that lack it.
    ///
    /// *Note*: Under most circumstances, you’ll want to use `get_field` (or
    /// its shorthand `g`) or nth rather than bracket. The bracket term may
    /// be useful in situations where you are unsure of the data type
    /// returned by the term you are calling bracket on.
    ///
    /// ## Example
    /// What was Iron Man’s first appearance in a comic?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").bracket("firstAppearance").run(conn)
    /// # })
    /// ```
    ///
    /// The `bracket` command also accepts integer arguments as array offsets,
    /// like the `nth` command.
    ///
    /// ## Example
    /// Get the fourth element in a sequence. (The first element is position 0,
    /// so the fourth element is position 3.)
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([10, 20, 30, 40, 50]).bracket(3).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [get_field](Self::get_field)
    /// - [nth](Self::nth)
    bracket(attr_or_index: Serialize)
);

create_cmd!(
    /// Get a single field from an object. If called on a sequence, gets that field from
    /// every object in the sequence, skipping objects that lack it.
    ///
    /// You may use either `get_field` or its shorthand, `g`.
    ///
    /// ## Example
    /// What was Iron Man’s first appearance in a comic?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").g("firstAppearance").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [nth](Self::nth)
    g:GetField(attr: Serialize)
    get_field(attr: Serialize)
);

create_cmd!(
    /// Test if an object has one or more fields. An object has a field if it
    /// has that key and the key has a non-null value. For instance, the object
    /// `{'a': 1,'b': 2,'c': null}` has the fields `a` and `b`.
    ///
    /// When applied to a single object, `has_fields` returns `true` if the
    /// object has the fields and false if it does not. When applied to
    /// a sequence, it will return a new sequence (an array or stream)
    /// containing the elements that have the specified fields.
    ///
    /// ## Example
    /// Return the players who have won games.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").has_fields("games_won").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the players who have not won games. To do this, use `has_fields`
    /// with not, wrapped with `filter`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").filter(r.row().has_fields("games_won").not()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Test if a specific player has won any games.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").get(1).has_fields("games_won").run(conn)
    /// # })
    /// ```
    ///
    /// ## Nested Fields
    ///
    /// `has_fields` lets you test for nested fields in objects. If the value of a field is itself a set of key/value pairs, you can test for the presence of specific keys.
    ///
    /// ## Example
    /// In the `players` table, the `games_won` field contains one or more fields for kinds of games won:
    ///
    /// ```json
    /// {
    ///     "games_won": {
    ///         "playoffs": 2,
    ///         "championships": 1
    ///     }
    /// }
    /// ```
    ///
    /// Return players who have the “championships” field.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("players")
    ///   .has_fields(rjson!({"games_won": {"championships": true}}))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// *Note* that `true` in the example above is testing for the existence
    /// of `championships` as a field, not testing to see if the value of the
    /// `championships` field is set to `true`. There’s a more convenient
    /// shorthand form available. (See [pluck](Command::pluck) for more details on this.)
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("players")
    ///   .has_fields(rjson!({"games_won": "championships"}))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [with_fields](Self::with_fields)
    only_command,
    has_fields(selector: ManyArgs<()>)
);

create_cmd!(
    /// Insert a value in to an array at a given index. Returns the modified array.
    ///
    /// ## Example
    /// Hulk decides to join the avengers.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["Iron Man", "Spider-Man"]).insert_at(1, "Hulk").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [splice_at](Self::splice_at)
    /// - [delete_at](Self::delete_at)
    /// - [change_at](Self::change_at)
    only_command,
    insert_at(offset: Serialize, value: Serialize)
);

create_cmd!(
    /// Insert several values in to an array at a given index. Returns the modified array.
    ///
    /// ## Example
    /// Hulk and Thor decide to join the avengers.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["Iron Man", "Spider-Man"]).splice_at(1, ["Hulk", "Thor"]).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [insert_at](Self::insert_at)
    /// - [delete_at](Self::delete_at)
    /// - [change_at](Self::change_at)
    only_command,
    splice_at(offset: Serialize, array: Serialize)
);

create_cmd!(
    /// Remove one or more elements from an array at a given index. Returns the
    /// modified array. (Note: `delete_at` operates on arrays, not documents; to
    /// delete documents, see the [delete](Self::delete) command.)
    ///
    /// If only `offset` is specified, `delete_at` removes the element at that
    /// index. If both `offset` and `endOffset` are specified, `delete_at`
    /// removes the range of elements between `offset` and `endOffset`,
    /// inclusive of `offset` but not inclusive of `endOffset`.
    ///
    /// If `endOffset` is specified, it must not be less than offset. Both
    /// `offset` and `endOffset` must be within the array’s bounds (i.e., if
    /// the array has 10 elements, an `offset` or `endOffset` of 10 or
    /// higher is invalid).
    ///
    /// By using a negative `offset` you can delete from the end of the array.
    /// `-1` is the last element in the array, -2 is the second-to-last element,
    /// and so on. You may specify a negative `endOffset`, although just as
    /// with a positive value, this will not be inclusive. The range `(2,-1)`
    /// specifies the third element through the next-to-last element.
    ///
    /// ## Example
    /// Delete the second element of an array.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["a","b","c","d","e","f"]).delete_at(1).run(conn)
    /// // Result: ["a", "c", "d", "e", "f"]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Delete the second and third elements of an array.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["a","b","c","d","e","f"]).delete_at(r.args((1, 3))).run(conn)
    /// // Result: ["a", "d", "e", "f"]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Delete the next-to-last element of an array.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["a","b","c","d","e","f"]).delete_at(-2).run(conn)
    /// // Result: ["a", "b", "c", "d", "f"]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Delete a comment on a post.
    /// Given a post document such as:
    ///
    /// ```json
    /// {
    ///     "id": "4cf47834-b6f9-438f-9dec-74087e84eb63",
    ///     "title": "Post title",
    ///     "author": "Bob",
    ///     "comments": [
    ///         { "author": "Agatha", "text": "Comment 1" },
    ///         { "author": "Fred", "text": "Comment 2" }
    ///     ]
    /// }
    /// ```
    ///
    /// The second comment can be deleted by using update and deleteAt together.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .get("4cf47834-b6f9-438f-9dec-74087e84eb63")
    ///   .update(rjson!({ "comments": r.row().g("comments").delete_at(1) }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [insert_at](Self::insert_at)
    /// - [splice_at](Self::splice_at)
    /// - [change_at](Self::change_at)
    only_command,
    delete_at(start_end_offset: OneAndSecondOptionalArg<()>)
);

create_cmd!(
    /// Change a value in an array at a given index. Returns the modified array.
    ///
    /// ## Example
    /// Bruce Banner hulks out.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["Iron Man", "Bruce", "Spider-Man"]).change_at(1, "Hulk").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [insert_at](Self::insert_at)
    /// - [splice_at](Self::splice_at)
    /// - [delete_at](Self::delete_at)
    only_command,
    change_at(offset: Serialize, value: Serialize)
);

create_cmd!(
    /// Return an array containing all of an object’s keys. Note that the keys will be sorted as described in ReQL data types (for strings, lexicographically).
    ///
    /// ## Example
    /// Get all the keys from a table row.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// // row: { id: 1, mail: "fred@example.com", name: "fred" }
    ///
    /// r.table("users").get(1).keys().run(conn)
    /// // Result: [ "id", "mail", "name" ]
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [insert_at](Self::insert_at)
    /// - [splice_at](Self::splice_at)
    /// - [delete_at](Self::delete_at)
    only_command,
    keys
);

create_cmd!(
    /// Return an array containing all of an object’s values. values() guarantees the values will come out in the same order as keys.
    ///
    /// ## Example
    /// Get all of the values from a table row.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// // row: { id: 1, mail: "fred@example.com", name: "fred" }
    ///
    /// r.table("users").get(1).values().run(conn)
    /// // Result: [ 1, "fred@example.com", "fred" ]
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [insert_at](Self::insert_at)
    /// - [splice_at](Self::splice_at)
    /// - [delete_at](Self::delete_at)
    only_command,
    values
);

create_cmd!(
    /// Replace an object in a field instead of merging it with an existing
    /// object in a `merge` or `update` operation. Using `literal` with no
    /// arguments in a `merge` or `update` operation will remove the
    /// corresponding field.
    ///
    /// Assume your users table has this structure:
    ///
    /// ```json
    /// [
    ///     {
    ///         "id": 1,
    ///         "name": "Alice",
    ///         "data": {
    ///             "age": 18,
    ///             "city": "Dallas"
    ///         }
    ///     }
    ///     ...
    /// ]
    /// ```
    ///
    /// Using `update` to modify the `data` field will normally merge
    /// the nested documents:
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").get(1).update(rjson!({ "data": { "age": 19, "job": "Engineer" } })).run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// {
    ///     "id": 1,
    ///     "name": "Alice",
    ///     "data": {
    ///         "age": 19,
    ///         "city": "Dallas",
    ///         "job": "Engineer"
    ///     }
    /// }
    /// ```
    ///
    /// That will preserve `city` and other existing fields. But to replace
    /// the entire `data` document with a new object, use `literal`.
    ///
    /// ## Example
    /// Replace one nested document with another rather than merging the fields.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .get(1)
    ///   .update(rjson!({ "data": r.literal(rjson!({ "age": 19, "job": "Engineer" })) }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// {
    ///     "id": 1,
    ///     "name": "Alice",
    ///     "data": {
    ///         "age": 19,
    ///         "job": "Engineer"
    ///     }
    /// }
    /// ```
    ///
    /// ## Example
    /// Use literal to remove a field from a document.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .get(1)
    ///   .merge(rjson!({ "data": r.literal(()) }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// {
    ///     "id": 1,
    ///     "name": "Alice"
    /// }
    /// ```
    ///
    /// # Related commands
    /// - [merge](Self::merge)
    /// - [filter](Self::filter)
    only_root,
    literal(object: Serialize)
);

create_cmd!(
    /// Creates an object from a list of key-value pairs, where the keys must
    /// be strings. `r.object(A, B, C, D)` is equivalent to
    /// `r.expr([[A, B], [C, D]]).coerce_to('OBJECT')`.
    ///
    /// ## Example
    /// Create a simple object.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.object(r.args(("id", 5, "data", ["foo", "bar"]))).run(conn)
    /// // Result: {"data": ["foo", "bar"], "id": 5}
    /// # })
    /// ```
    ///
    /// or for many args, use array of `serde_json::Value`:
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.object(r.args([rjson!("id"), rjson!(5), rjson!("data"), rjson!(["foo", "bar"])])).run(conn)
    /// // Result: {"data": ["foo", "bar"], "id": 5}
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [coerce_to](Self::coerce_to)
    /// - [merge](Self::merge)
    /// - [keys](Self::keys)
    only_root,
    object(key_value: ManyArgs<()>)
);
