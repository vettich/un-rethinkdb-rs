use unreql_macros::create_cmd;
use ql2::term::TermType;
use serde::Serialize;

use crate::{
    cmd::{args::ManyArgs, options::RandomOptions},
    Command,
};

create_cmd!(
    /// Sum two or more numbers, or concatenate two or more strings or arrays.
    ///
    /// The `add` command can be called in either prefix or infix form; both
    /// forms are equivalent. Note that ReQL will not perform type coercion.
    /// You cannot, for example, `add` a string and a number together.
    ///
    /// ## Example
    /// It’s as easy as 2 + 2 = 4.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(2).add(2).run(conn)
    /// // Result: 4
    /// # })
    /// ```
    ///
    /// ## Example
    /// Concatenate strings.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("foo").add(r.args(["bar", "baz"])).run(conn)
    /// // Result: "foobarbaz"
    /// # })
    /// ```
    ///
    /// ## Example
    /// Concatenate arrays.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["foo", "bar"]).add(["buzz"]).run(conn)
    /// // Result: [ "foo", "bar", "buzz" ]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a date one year from now.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.now().add(365 * 24 * 60 * 60).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Use args with add to sum multiple values.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let vals = vec![10, 20, 30];
    /// r.expr(0).add(r.args(&vals)).run(conn)
    /// // Result: 60
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [sub](Self::sub)
    /// - [mul](Self::mul)
    /// - [div](Self::div)
    /// - [mod_](Self::mod_)
    only_command,
    add(value: ManyArgs<()>)
);

create_cmd!(
    /// Subtract two numbers.
    ///
    /// ## Example
    /// It’s as easy as 2 - 2 = 0.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(2).sub(2).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a date one year ago today.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.now().sub(365*24*60*60).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Retrieve how many seconds elapsed between today and `date`.
    ///
    /// ```
    /// # use unreql::DateTime;
    /// # use time::OffsetDateTime;
    /// # unreql::example(|r, conn| {
    /// let date = DateTime::from(OffsetDateTime::now_utc());
    /// r.now().sub(date).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [add](Self::add)
    /// - [mul](Self::mul)
    /// - [div](Self::div)
    /// - [mod_](Self::mod_)
    only_command,
    sub(value: ManyArgs<()>)
);

create_cmd!(
    /// Multiply two numbers, or make a periodic array.
    ///
    /// ## Example
    /// It’s as easy as 2 * 2 = 4.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(2).mul(2).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Arrays can be multiplied by numbers as well.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["This", "is", "the", "song", "that", "never", "ends."]).mul(100).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [add](Self::add)
    /// - [sub](Self::sub)
    /// - [div](Self::div)
    /// - [mod_](Self::mod_)
    only_command,
    mul(number: ManyArgs<()>)
);

create_cmd!(
    /// Divide two numbers.
    ///
    /// ## Example
    /// It’s as easy as 2 / 2 = 1.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(2).div(2).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [add](Self::add)
    /// - [sub](Self::sub)
    /// - [mul](Self::mul)
    /// - [mod_](Self::mod_)
    only_command,
    div(number: ManyArgs<()>)
);

create_cmd!(
    /// Find the remainder when dividing two numbers.
    ///
    /// ## Example
    /// It’s as easy as 2 % 2 = 0.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(2).mod_(2).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [add](Self::add)
    /// - [sub](Self::sub)
    /// - [mul](Self::mul)
    /// - [div](Self::div)
    only_command,
    mod_(number: ManyArgs<()>)
);

create_cmd!(
    /// Compute the logical “and” of one or more values.
    ///
    /// The `and` command can be used as an infix operator after its first
    /// argument (`r.expr(true).and(false)`) or given all of its arguments
    /// as parameters (`r.and(true,false)`).
    ///
    /// Calling `and` with zero arguments will return `true`.
    ///
    /// ## Example
    /// Return whether both `a` and `b` evaluate to true.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let a = true;
    /// let b = false;
    /// r.expr(a).and(b).run(conn)
    /// // Result: false
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [or](Self::or)
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    and(boolean: ManyArgs<()>)
);

create_cmd!(
    /// Compute the logical “or” of one or more values.
    ///
    /// The `or` command can be used as an infix operator after its first
    /// argument (`r.expr(true).or(false)`) or given all of its arguments
    /// as parameters (`r.or(true,false)`).
    ///
    /// Calling `or` with zero arguments will return `false`.
    ///
    /// ## Example
    /// Return whether either `a` or `b` evaluate to true.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let a = true;
    /// let b = false;
    /// r.expr(a).or(b).run(conn)
    /// // Result: true
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return whether any of `a` or `b` evaluate to true.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let a = false;
    /// let b = false;
    /// r.or(r.args([a, b])).run(conn)
    /// // Result: false
    /// # })
    /// ```
    ///
    /// *Note*: When using `or` inside a `filter` predicate to test the values
    /// of fields that may not exist on the documents being tested, you should
    /// use the `default` command with those fields so they explicitly return
    /// `false`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").filter(
    ///   r.row().g("category").default("foo").eq("article")
    ///     .or(r.row().g("category").default("foo").eq("mystery"))
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [and](Self::and)
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    or(boolean: ManyArgs<()>)
);

create_cmd!(
    /// Test if two or more values are equal.
    ///
    /// ## Example
    /// See if a user’s role field is set to administrator.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").get(1).g("role").eq("administrator").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// See if three variables contain equal values.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// # let (a, b, c) = (true, true, true);
    /// r.eq(r.args([a, b, c])).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [and](Self::and)
    /// - [or](Self::or)
    /// - [ne](Self::ne)
    eq(value: ManyArgs<()>)
);

create_cmd!(
    /// Test if two or more values are not equal.
    ///
    /// ## Example
    /// See if a user’s role field is not set to administrator.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").get(1).g("role").ne("administrator").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// See if three variables do not contain equal values.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// # let (a, b, c) = (true, true, true);
    /// r.ne(r.args([a, b, c])).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [and](Self::and)
    /// - [or](Self::or)
    /// - [eq](Self::eq)
    ne(value: ManyArgs<()>)
);

create_cmd!(
    /// Compare values, testing if the left-hand value is greater than
    /// the right-hand.
    ///
    /// ## Example
    /// Test if a player has scored more than 10 points.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").get(1).g("score").gt(10).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Test if variables are ordered from lowest to highest, with no values being equal to one another.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let (a, b, c) = (15, 20, 15);
    /// r.gt(r.args([a, b, c])).run(conn)
    /// # })
    /// ```
    ///
    /// This is the equivalent of the following:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// # let (a, b, c) = (15, 20, 15);
    /// r.gt(r.args([a, b])).and(r.gt(r.args([b, c]))).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [ge](Self::ge)
    /// - [lt](Self::lt)
    /// - [le](Self::le)
    gt(value: ManyArgs<()>)
);

create_cmd!(
    /// Compare values, testing if the left-hand value is greater than or
    /// equal to the right-hand.
    ///
    /// ## Example
    /// Test if a player has scored 10 points or more.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").get(1).g("score").ge(10).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [lt](Self::lt)
    /// - [le](Self::le)
    ge(value: ManyArgs<()>)
);

create_cmd!(
    /// Compare values, testing if the left-hand value is less than
    /// the right-hand.
    ///
    /// ## Example
    /// Test if a player has scored less than 10 points.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").get(1).g("score").lt(10).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [ge](Self::ge)
    /// - [le](Self::le)
    lt(value: ManyArgs<()>)
);

create_cmd!(
    /// Compare values, testing if the left-hand value is less than or equal
    /// to the right-hand.
    ///
    /// ## Example
    /// Test if a player has scored 10 points or less.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").get(1).g("score").le(10).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    /// - [gt](Self::gt)
    /// - [ge](Self::ge)
    /// - [lt](Self::lt)
    le(value: ManyArgs<()>)
);

create_cmd!(
    /// Compute the logical inverse (not) of an expression.
    ///
    /// `not` can be called either via method chaining, immediately after
    /// an expression that evaluates as a boolean value, or by passing
    /// the expression as a parameter to `not`. All values that are not `false`
    /// or `null` will be converted to `true`.
    ///
    /// ## Example
    /// Not true is false.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.not(true).run(conn)
    /// // Result: false
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all the users that do not have a “flag” field.
    ///
    /// ```
    /// use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   r.not(user.has_fields("flag"))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    only_root,
    not(boolean: Serialize)
);

create_cmd!(
    /// Compute the logical inverse (not) of an expression.
    ///
    /// `not` can be called either via method chaining, immediately after
    /// an expression that evaluates as a boolean value, or by passing
    /// the expression as a parameter to `not`. All values that are not `false`
    /// or `null` will be converted to `true`.
    ///
    /// ## Example
    /// Not true is false.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(true).not().run(conn)
    /// // Result: false
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all the users that do not have a “flag” field.
    ///
    /// ```
    /// use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.has_fields("flag").not()
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [eq](Self::eq)
    /// - [ne](Self::ne)
    only_command,
    not,
);

create_cmd!(
    /// Generate a random number between given (or implied) bounds.
    /// `random` takes zero, one or two arguments.
    ///
    /// - With *zero* arguments, the result will be a floating-point number
    ///   in the range `[0,1)` (from 0 up to but not including 1).
    /// - With *one* argument `x`, the result will be in the range `[0,x)`,
    ///   and will be integer unless `{float: true}` is given as an option.
    ///   Specifying a floating point number without the `float` option will
    ///   raise an error.
    /// - With *two* arguments `x` and `y`, the result will be in the range
    ///   `[x,y)`, and will be integer unless `{float: true}` is given as an
    ///   option. If `x` and `y` are equal an error will occur, unless the
    ///   floating-point option has been specified, in which case `x` will
    ///   be returned. Specifying a floating point number without the `float`
    ///   option will raise an error.
    ///
    /// *Note*: The last argument given will always be the ‘open’ side of the
    /// range, but when generating a floating-point number, the ‘open’ side
    /// may be less than the ‘closed’ side.
    ///
    /// ## Example
    /// Generate a random number in the range `[0,1)`
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.random(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Generate a random integer in the range `[0,100)`
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.random(100).run(conn)
    /// # });
    /// # unreql::example(|r, conn| {
    /// r.random(r.args([0, 100])).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Generate a random number in the range (-2.24,1.59]
    ///
    /// ```
    /// # use unreql::cmd::options::RandomOptions;
    /// # unreql::example(|r, conn| {
    /// let opts = RandomOptions::new().float(true);
    /// r.random(r.with_opt(r.args([1.59, -2.24]), opts)).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [sample](Self::sample)
    random(numbers: ManyArgs<RandomOptions>)
);

create_cmd!(
    /// Rounds the given value to the nearest whole integer.
    ///
    /// For example, values of 1.0 up to but not including 1.5 will return 1.0,
    /// similar to `floor`; values of 1.5 up to 2.0 will return 2.0, similar
    /// to `ceil`.
    ///
    /// ## Example
    /// Round 12.345 to the nearest integer.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.round(12.345).run(conn)
    /// // Result: 12.0
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [ceil](Self::ceil)
    /// - [floor](Self::floor)
    only_root,
    round(number: Serialize)
);

create_cmd!(
    /// Rounds the given value to the nearest whole integer.
    ///
    /// For example, values of 1.0 up to but not including 1.5 will return 1.0,
    /// similar to `floor`; values of 1.5 up to 2.0 will return 2.0, similar
    /// to `ceil`.
    ///
    /// ## Example
    /// Round -12.345 to the nearest integer.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(-12.345).round().run(conn)
    /// // Result: -12.0
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return Iron Man’s weight, rounded to the nearest integer.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("superheroes").get("ironman").g("weight").round().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [ceil](Self::ceil)
    /// - [floor](Self::floor)
    only_command,
    round
);

create_cmd!(
    /// Rounds the given value up, returning the smallest integer value greater
    /// than or equal to the given value (the value’s ceiling).
    ///
    /// ## Example
    /// Return the ceiling of 12.345.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.ceil(12.345).run(conn)
    /// // Result: 13.0
    /// # })
    /// ```
    ///
    /// The `ceil` command can also be chained after an expression.
    ///
    /// # Related commands
    /// - [round](Self::round)
    /// - [floor](Self::floor)
    only_root,
    ceil(number: Serialize)
);

create_cmd!(
    /// Rounds the given value up, returning the smallest integer value greater
    /// than or equal to the given value (the value’s ceiling).
    ///
    /// ## Example
    /// Return the ceiling of -12.345.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(-12.345).round().run(conn)
    /// // Result: -12.0
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return Iron Man’s weight, rounded up with ceil.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("superheroes").get("ironman").g("weight").ceil().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [round](Self::round)
    /// - [floor](Self::floor)
    only_command,
    ceil
);

create_cmd!(
    /// Rounds the given value down, returning the largest integer value less
    /// than or equal to the given value (the value’s floor).
    ///
    /// ## Example
    /// Return the floor of 12.345.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.floor(12.345).run(conn)
    /// // Result: 12.0
    /// # })
    /// ```
    ///
    /// The floor command can also be chained after an expression.
    ///
    /// # Related commands
    /// - [round](Self::round)
    /// - [ceil](Self::ceil)
    only_root,
    floor(number: Serialize)
);

create_cmd!(
    /// Rounds the given value down, returning the largest integer value less
    /// than or equal to the given value (the value’s floor).
    ///
    /// ## Example
    /// Return the floor of -12.345.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(-12.345).floor().run(conn)
    /// // Result: -13.0
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return Iron Man’s weight, rounded up with floor.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("superheroes").get("ironman").g("weight").floor().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [round](Self::round)
    /// - [ceil](Self::ceil)
    only_command,
    floor
);

create_cmd!(
    /// A bitwise AND is a binary operation that takes two equal-length binary
    /// representations and performs the logical AND operation on each pair
    /// of the corresponding bits, which is equivalent to multiplying them.
    /// Thus, if both bits in the compared position are 1, the bit in the
    /// resulting binary representation is 1 (1 × 1 = 1); otherwise, the
    /// result is 0 (1 × 0 = 0 and 0 × 0 = 0).
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(5).bit_and(3).run(conn)
    /// // Result: 1
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [bit_not](Self::bit_not)
    /// - [bit_or](Self::bit_or)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    bit_and(number: ManyArgs<()>)
);

create_cmd!(
    /// A bitwise OR is a binary operation that takes two bit patterns of equal
    /// length and performs the logical inclusive OR operation on each pair of
    /// corresponding bits. The result in each position is 0 if both bits are
    /// 0, while otherwise the result is 1.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(5).bit_or(3).run(conn)
    /// // Result: 7
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [bit_not](Self::bit_not)
    /// - [bit_and](Self::bit_and)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    bit_or(number: ManyArgs<()>)
);

create_cmd!(
    /// A bitwise XOR is a binary operation that takes two bit patterns of
    /// equal length and performs the logical exclusive OR operation on each
    /// pair of corresponding bits. The result in each position is 1 if only
    /// the first bit is 1 or only the second bit is 1, but will be 0 if both
    /// are 0 or both are 1. In this we perform the comparison of two bits,
    /// being 1 if the two bits are different, and 0 if they are the same.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(6).bit_xor(4).run(conn)
    /// // Result: 2
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [bit_not](Self::bit_not)
    /// - [bit_and](Self::bit_and)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    bit_xor(number: ManyArgs<()>)
);

create_cmd!(
    /// A bitwise NOT, or complement, is a unary operation that performs
    /// logical negation on each bit, forming the ones’ complement of the
    /// given binary value. Bits that are 0 become 1, and those that
    /// are 1 become 0.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(7).bit_not().run(conn)
    /// // Result: -8
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [bit_and](Self::bit_and)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    /// - [bit_sar](Self::bit_sar)
    bit_not
);

create_cmd!(
    /// In an arithmetic shift (also referred to as signed shift), like
    /// a logical shift, the bits that slide off the end disappear (except
    /// for the last, which goes into the carry flag). But in an arithmetic
    /// shift, the spaces are filled in such a way to preserve the sign of
    /// the number being slid. For this reason, arithmetic shifts are
    /// better suited for signed numbers in two’s complement format.
    ///
    /// *Note*: SHL and SAL are the same, and differentiation only happens
    /// because SAR and SHR (right shifting) has differences in their
    /// implementation.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(5).bit_sal(4).run(conn)
    /// // Result: 80
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [bit_not](Self::bit_not)
    /// - [bit_and](Self::bit_and)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sar](Self::bit_sar)
    bit_sal(number: ManyArgs<()>)
);

create_cmd!(
    /// In an arithmetic shift (also referred to as signed shift), like
    /// a logical shift, the bits that slide off the end disappear (except
    /// for the last, which goes into the carry flag). But in an arithmetic
    /// shift, the spaces are filled in such a way to preserve the sign of
    /// the number being slid. For this reason, arithmetic shifts are better
    /// suited for signed numbers in two’s complement format.
    ///
    /// ## Example
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(32).bit_sal(3).run(conn)
    /// // Result: 4
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [bit_not](Self::bit_not)
    /// - [bit_and](Self::bit_and)
    /// - [bit_xor](Self::bit_xor)
    /// - [bit_or](Self::bit_or)
    /// - [bit_sal](Self::bit_sal)
    bit_sar(number: ManyArgs<()>)
);
