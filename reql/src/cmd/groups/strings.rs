use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{cmd::args::ManyArgs, Command};

create_cmd!(
    /// Matches against a regular expression.
    ///
    /// If there is a match, returns an object with the fields:
    ///
    /// - `str`: The matched string
    /// - `start`: The matched string’s start
    /// - `end`: The matched string’s end
    /// - `groups`: The capture groups defined with parentheses
    ///
    /// If no match is found, returns `null`.
    ///
    /// Accepts [RE2 syntax](https://github.com/google/re2/wiki/Syntax).
    /// You can enable case-insensitive matching by prefixing the regular
    /// expression with `(?i)`. See the linked RE2 documentation for more flags.
    ///
    /// The `match` command does not support backreferences.
    ///
    /// ## Example
    /// Get all users whose name starts with “A”. Because `null` evaluates
    /// to `false` in `filter`, you can just use the result of `match` for the
    /// predicate.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .filter(func!(|doc| doc.g("name").match_("^A")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users whose name ends with “n”.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .filter(func!(|doc| doc.g("name").match_("n$")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users whose name has “li” in it
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .filter(func!(|doc| doc.g("name").match_("li")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users whose name is “John” with a case-insensitive search.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .filter(func!(|doc| doc.g("name").match_("(?i)^john$")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users whose name is composed of only characters between “a” and “z”.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .filter(func!(|doc| doc.g("name").match_("(?i)^[a-z]+$")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users where the zipcode is a string of 5 digits.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users")
    ///   .filter(func!(|doc| doc.g("zipcode").match_("\\d{5}")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Retrieve the domain of a basic email
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("name@domain.com").match_(".*@(.*)").run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// {
    ///     "start": 0,
    ///     "end": 20,
    ///     "str": "name@domain.com",
    ///     "groups": [
    ///         {
    ///             "end": 17,
    ///             "start": 7,
    ///             "str": "domain.com"
    ///         }
    ///     ]
    /// }
    /// ```
    ///
    /// You can then retrieve only the domain with the `g` selector and `nth`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("name@domain.com").match_(".*@(.*)").g("groups").nth(0).g("str").run(conn)
    /// // Return: "domain.com"
    /// # })
    /// ```
    ///
    /// ## Example
    /// Fail to parse out the domain and returns `null`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("name[at]domain.com").match_(".*@(.*)").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [upcase](Self::upcase)
    /// - [downcase](Self::downcase)
    /// - [split](Self::split)
    only_command,
    match_(regexp: Serialize)
);

create_cmd!(
    /// Splits a string into substrings. Splits on whitespace when called
    /// with no arguments. When called with a separator, splits on that
    /// separator. When called with a separator and a maximum number of
    /// splits, splits on that separator at most `max_splits` times. (Can be
    /// called with `null` as the separator if you want to split on whitespace
    /// while still specifying `max_splits`.)
    ///
    /// Mimics the behavior of Python’s `string.split` in edge cases, except
    /// for splitting on the empty string, which instead produces an array
    /// of single-character strings.
    ///
    /// ## Example
    /// Split on whitespace.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("foo  bar bax").split(()).run(conn)
    /// // Result: ["foo", "bar", "bax"]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Split the entries in a CSV file.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("12,37,,22,").split(",").run(conn)
    /// // Result: ["12", "37", "", "22", ""]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Split a string into characters.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("mlucy").split("").run(conn)
    /// // Result: ["m", "l", "u", "c", "y"]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Split the entries in a CSV file, but only at most 3 times.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("12,37,,22,").split(r.args((",", 3))).run(conn)
    /// // Result: ["12", "37", "", "22,"]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Split on whitespace at most once (i.e. get the first word).
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.expr("foo  bar bax").split(r.args((rjson!(null), 1))).run(conn)
    /// // Result: ["foo", "bar bax"]
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [upcase](Self::upcase)
    /// - [downcase](Self::downcase)
    /// - [match_](Self::match_)
    split(separator_and_max_splits: ManyArgs<()>)
);

create_cmd!(
    /// Uppercases a string.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("Sentence about LaTeX.").upcase().run(conn)
    /// // Result: "SENTENCE ABOUT LATEX."
    /// # })
    /// ```
    ///
    /// *Note*: `upcase` and `downcase` only affect ASCII characters.
    ///
    /// # Related commands
    /// - [upcase](Self::upcase)
    /// - [downcase](Self::downcase)
    /// - [match_](Self::match_)
    upcase
);

create_cmd!(
    /// Lowercases a string.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("Sentence about LaTeX.").upcase().run(conn)
    /// // Result: "sentence about latex."
    /// # })
    /// ```
    ///
    /// *Note*: `upcase` and `downcase` only affect ASCII characters.
    ///
    /// # Related commands
    /// - [upcase](Self::upcase)
    /// - [downcase](Self::downcase)
    /// - [match_](Self::match_)
    downcase
);
