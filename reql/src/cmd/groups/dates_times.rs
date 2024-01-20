use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::{args::Opt, options::DuringOptions},
    Command,
};

create_cmd!(
    /// Return a time object representing the current time in UTC.
    ///
    /// The command `now()` is computed once when the server receives the query,
    /// so multiple instances of `r.now()` will always return the same time
    /// inside a query.
    ///
    /// ## Example
    /// Add a new user with the time at which he subscribed.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").insert(rjson!({
    ///   "name": "John",
    ///   "subscription_date": r.now(),
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [time](Self::time)
    /// - [epoch_time](Self::epoch_time)
    /// - [iso_8601](Self::iso_8601)
    only_root,
    now,
);

create_cmd!(
    /// Create a time object for a specific time.
    ///
    /// A few restrictions exist on the arguments:
    ///
    /// - `year` is an integer between 1400 and 9,999.
    /// - `month` is an integer between 1 and 12.
    /// - `day` is an integer between 1 and 31.
    /// - `timezone` can be `'Z'` (for UTC) or a string with the format `±[hh]:[mm]`.
    ///
    /// To use `hour`, `minutes` and `seconds` see [time_ext](Self::time_ext) command.
    ///
    /// ## Example
    /// Update the birthdate of the user “John” to November 3rd, 1986 UTC.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").get("John").update(rjson!({
    ///   "birthdate": r.time(1986, 11, 3, "Z"),
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [epoch_time](Self::epoch_time)
    /// - [iso_8601](Self::iso_8601)
    only_root,
    time(year: Serialize, month: Serialize, day: Serialize, timezone: Serialize)
);

create_cmd!(
    /// Create a time object for a specific time.
    ///
    /// A few restrictions exist on the arguments:
    ///
    /// - `year` is an integer between 1400 and 9,999.
    /// - `month` is an integer between 1 and 12.
    /// - `day` is an integer between 1 and 31.
    /// - `hour` is an integer.
    /// - `minutes` is an integer.
    /// - `seconds` is a double. Its value will be rounded to three decimal places (millisecond-precision).
    /// - `timezone` can be `'Z'` (for UTC) or a string with the format `±[hh]:[mm]`.
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [epoch_time](Self::epoch_time)
    /// - [iso_8601](Self::iso_8601)
    only_root,
    time_ext:Time(year: Serialize, month: Serialize, day: Serialize, hour: Serialize, minutes: Serialize, seconds: Serialize, timezone: Serialize)
);

create_cmd!(
    /// Create a time object based on seconds since epoch.
    ///
    /// The first argument is a double and will be rounded to three decimal
    /// places (millisecond-precision).
    ///
    /// ## Example
    /// Update the birthdate of the user “John” to November 3rd, 1986.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").get("John").update(rjson!({
    ///   "birthdate": r.epoch_time(531360000),
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [iso_8601](Self::iso_8601)
    only_root,
    epoch_time(number: Serialize)
);

create_cmd!(
    /// Create a time object based on an ISO 8601 date-time string
    /// (e.g. ‘2013-01-01T01:01:01+00:00’).
    ///
    /// RethinkDB supports all valid ISO 8601 formats except for week dates.
    /// Read more about the ISO 8601 format at [Wikipedia](http://en.wikipedia.org/wiki/ISO_8601).
    ///
    /// ## Example
    /// Update the time of John’s birth.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").get("John").update(rjson!({
    ///   "birthdate": r.iso_8601("1986-11-03T08:30:00-07:00"),
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [epoch_time](Self::epoch_time)
    only_root,
    iso_8601(string: Serialize)
);

create_cmd!(
    /// Return a new time object with a different timezone.
    ///
    /// While the time stays the same, the results returned by methods such
    /// as `hours()` will change since they take the timezone into account.
    /// The timezone argument has to be of the ISO 8601 format.
    ///
    /// ## Example
    /// Hour of the day in San Francisco (UTC/GMT -8, without daylight saving time).
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.now().in_timezone("-08:00").hours().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [timezone](Self::timezone)
    only_command,
    in_timezone(timezone: Serialize)
);

create_cmd!(
    /// Return the timezone of the time object.
    ///
    /// ## Example
    /// Return all the users in the “-07:00” timezone.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.row().g("subscriptionDate").timezone().eq("-07:00")).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [in_timezone](Self::in_timezone)
    only_command,
    timezone
);

create_cmd!(
    /// Return whether a time is between two other times.
    ///
    /// By default, this is inclusive of the start time and exclusive of the
    /// end time. Set `leftBound` and `rightBound` to explicitly include
    /// (`closed`) or exclude (`open`) that endpoint of the range.
    ///
    /// ## Example
    /// Retrieve all the posts that were posted between December 1st, 2013
    /// (inclusive) and December 10th, 2013 (exclusive).
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").filter(
    ///   r.row()
    ///     .g("date")
    ///     .during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"), ())
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Retrieve all the posts that were posted between December 1st, 2013 (exclusive) and December 10th, 2013 (inclusive).
    ///
    /// ```
    /// # use unreql::cmd::options::{Status, DuringOptions};
    /// # unreql::example(|r, conn| {
    /// let opts = DuringOptions::new()
    ///   .left_bound(Status::Open)
    ///   .right_bound(Status::Closed);
    /// r.table("posts").filter(
    ///   r.row().g("date").during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"), opts)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [in_timezone](Self::in_timezone)
    only_command,
    during(start_time: Serialize, end_time: Serialize, opts: Opt<DuringOptions>)
);

create_cmd!(
    /// Return a new time object only based on the day, month and year
    /// (ie. the same day at 00:00).
    ///
    /// ## Example
    /// Retrieve all the users whose birthday is today.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.g("birthdate").date().eq(r.now().date())
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// *Note* that the `now` command always returns UTC time, so the comparison
    /// may fail if `user.g("birthdate")` isn’t also in UTC. You can use the
    /// `in_timezone` command to adjust for this:
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.g("birthdate").date().eq(r.now().in_timezone("-08:00").date())
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [in_timezone](Self::in_timezone)
    only_command,
    date
);

create_cmd!(
    /// Return the number of seconds elapsed since the beginning of the day
    /// stored in the time object.
    ///
    /// ## Example
    /// Retrieve posts that were submitted before noon.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("date").time_of_day().le(12*60*60)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [in_timezone](Self::in_timezone)
    time_of_day
);

create_cmd!(
    /// Return the year of a time object.
    ///
    /// ## Example
    /// Retrieve all the users born in 1986.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("birthdate").year().eq(1986)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    year,
);

create_cmd!(
    /// Return the month of a time object as a number between 1 and 12.
    /// For your convenience, the terms `r.january()`, `r.february()` etc. are
    /// defined and map to the appropriate integer.
    ///
    /// ## Example
    /// Retrieve all the users who were born in November.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("birthdate").month().eq(11)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Retrieve all the users who were born in November.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("birthdate").month().eq(r.november())
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [january](Self::january)
    month,
);

create_cmd!(
    /// Return the day of a time object as a number between 1 and 31.
    ///
    /// ## Example
    /// Return the users born on the 24th of any month.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("birthdate").day().eq(24)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    day,
);

create_cmd!(
    /// Return the day of week of a time object as a number between 1 and 7
    /// (following ISO 8601 standard). For your convenience, the terms
    /// `r.monday()`, `r.tuesday()` etc. are defined and map to the
    /// appropriate integer.
    ///
    /// ## Example
    /// Return today’s day of week.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.now().day_of_week().run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Retrieve all the users who were born on a Tuesday.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("birthdate").day_of_week().eq(r.tuesday())
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    /// - [monday](Self::monday)
    only_command,
    day_of_week
);

create_cmd!(
    /// Return the day of the year of a time object as a number between
    /// 1 and 366 (following ISO 8601 standard).
    ///
    /// ## Example
    /// Retrieve all the users who were born the first day of a year.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("birthdate").day_of_year().eq(1)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    only_command,
    day_of_year
);

create_cmd!(
    /// Return the hour in a time object as a number between 0 and 23.
    ///
    /// ## Example
    /// Return all the posts submitted after midnight and before 4am.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").filter(
    ///   r.row().g("date").hours().lt(4)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    only_command,
    hours
);

create_cmd!(
    /// Return the minute in a time object as a number between 0 and 59.
    ///
    /// ## Example
    /// Return all the posts submitted during the first 10 minutes of every hour.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").filter(
    ///   r.row().g("date").minutes().lt(10)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    only_command,
    minutes
);

create_cmd!(
    /// Return the seconds in a time object as a number between 0 and 59.999 (double precision).
    ///
    /// ## Example
    /// Return the post submitted during the first 30 seconds of every minute.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").filter(
    ///   r.row().g("date").seconds().lt(30)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    only_command,
    seconds
);

create_cmd!(
    /// Convert a time object to a string in ISO 8601 format.
    ///
    /// ## Example
    /// Return the current ISO 8601 time.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.now().to_iso_8601().run(conn)
    /// // Result: "2015-04-20T18:37:52.690+00:00"
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    only_command,
    to_iso_8601
);

create_cmd!(
    /// Convert a time object to its epoch time.
    ///
    /// ## Example
    /// Return the current time in seconds since the Unix Epoch with millisecond-precision.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.now().to_epoch_time().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [now](Self::now)
    /// - [time](Self::time)
    only_command,
    to_epoch_time
);

create_cmd!(only_root, monday);
create_cmd!(only_root, tuesday);
create_cmd!(only_root, wednesday);
create_cmd!(only_root, thursday);
create_cmd!(only_root, friday);
create_cmd!(only_root, saturday);
create_cmd!(only_root, sunday);

create_cmd!(only_root, january);
create_cmd!(only_root, february);
create_cmd!(only_root, march);
create_cmd!(only_root, april);
create_cmd!(only_root, may);
create_cmd!(only_root, june);
create_cmd!(only_root, july);
create_cmd!(only_root, august);
create_cmd!(only_root, september);
create_cmd!(only_root, october);
create_cmd!(only_root, november);
create_cmd!(only_root, december);
