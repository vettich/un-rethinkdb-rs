use std::{borrow::Cow, collections::HashMap};

use serde::{Serialize, Serializer};
use serde_with::skip_serializing_none;
use unreql_macros::{OptionsBuilder, WithOpts};

use crate::Command;

use super::args;

#[derive(Debug)]
pub struct Index(pub(crate) Command);

impl args::WithOpts for Index {
    fn with_opts(self, cmd: Command) -> Command {
        cmd.with_opts(self.0)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Copy, Default, Serialize, WithOpts, OptionsBuilder)]
pub struct RandomOptions {
    pub float: Option<bool>,
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct FilterOptions {
    // TODO implement for `true`, `false` and `r.error()`
    pub default: Option<bool>,
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct TableOptions {
    pub read_mode: Option<ReadMode>,
    pub identifier_format: Option<IdentifierFormat>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum IdentifierFormat {
    Name,
    Uuid,
}

/// Optional arguments to `changes`
#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct ChangesOptions {
    /// Controls how change notifications are batched
    pub squash: Option<Squash>,
    /// The number of changes the server will buffer between client reads
    /// before it starts dropping changes and generates an error
    /// (default: 100,000).
    pub changefeed_queue_size: Option<u32>,
    /// If `true`, the changefeed stream will begin with the current contents
    /// of the table or selection being monitored. These initial results will
    /// have `new_val` fields, but no `old_val` fields. The initial results
    /// may be intermixed with actual changes, as long as an initial result
    /// for the changed document has already been given. If an initial result
    /// for a document has been sent and a change is made to that document
    /// that would move it to the unsent part of the result set (e.g., a
    /// changefeed monitors the top 100 posters, the first 50 have been sent,
    /// and poster 48 has become poster 52), an "uninitial" notification will
    /// be sent, with an `old_val` field but no `new_val` field.
    pub include_initial: Option<bool>,
    /// If `true`, the changefeed stream will include special status documents
    /// consisting of the field `state` and a string indicating a change in the
    /// feed's state. These documents can occur at any point in the feed between
    /// the notification documents described below. If `includeStates` is `false`
    /// (the default), the status documents will not be sent.
    pub include_states: Option<bool>,
    /// If `true`, a changefeed stream on an `order_by.limit` changefeed will
    /// include `old_offset` and `new_offset` fields in status documents that
    /// include `old_val` and `new_val`. This allows applications to maintain
    /// ordered lists of the stream's result set. If `old_offset` is set and not
    /// `null`, the element at `old_offset` is being deleted; if `new_offset` is
    /// set and not `null`, then `new_val` is being inserted at `new_offset`.
    /// Setting `include_offsets` to `true` on a changefeed that does not support
    /// it will raise an error.
    pub include_offsets: Option<bool>,
    /// If `true`, every result on a changefeed will include a `type` field with
    /// a string that indicates the kind of change the result represents:
    /// `add`, `remove`, `change`, `initial`, `uninitial`, `state`.
    /// Defaults to `false`.
    ///
    /// There are currently two states:
    ///
    /// * `{state: 'initializing'}` indicates the following documents represent
    /// initial values on the feed rather than changes. This will be the first
    /// document of a feed that returns initial values.
    /// * `{state: 'ready'}` indicates the following documents represent changes.
    /// This will be the first document of a feed that does *not* return initial
    /// values; otherwise, it will indicate the initial values have all been sent.
    pub include_types: Option<bool>,
}

/// Controls how change notifications are batched
#[derive(Debug, Clone, Copy, Serialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Squash {
    /// `true`: When multiple changes to the same document occur before a
    /// batch of notifications is sent, the changes are "squashed" into one
    /// change. The client receives a notification that will bring it fully
    /// up to date with the server.
    /// `false`: All changes will be sent to the client verbatim. This is
    /// the default.
    Bool(bool),
    /// `n`: A numeric value (floating point). Similar to `true`, but the
    /// server will wait `n` seconds to respond in order to squash as many
    /// changes together as possible, reducing network traffic. The first
    /// batch will always be returned immediately.
    Float(f32),
}

#[derive(Debug, Clone, Default, PartialEq, WithOpts, OptionsBuilder)]
pub struct TableCreateOptions {
    pub primary_key: Option<Cow<'static, str>>,
    pub durability: Option<Durability>,
    pub shards: Option<u8>,
    pub replicas: Option<Replicas>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Replicas {
    Int(u8),
    Map {
        replicas: HashMap<Cow<'static, str>, u8>,
        primary_replica_tag: Cow<'static, str>,
    },
}

impl Serialize for TableCreateOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct InnerOptions<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_key: Option<&'a Cow<'static, str>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            durability: Option<Durability>,
            #[serde(skip_serializing_if = "Option::is_none")]
            shards: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            replicas: Option<InnerReplicas<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            primary_replica_tag: Option<&'a Cow<'static, str>>,
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum InnerReplicas<'a> {
            Int(u8),
            Map(&'a HashMap<Cow<'static, str>, u8>),
        }

        let (replicas, primary_replica_tag) = match &self.replicas {
            Some(Replicas::Int(i)) => (Some(InnerReplicas::Int(*i)), None),
            Some(Replicas::Map {
                replicas,
                primary_replica_tag,
            }) => (
                Some(InnerReplicas::Map(replicas)),
                Some(primary_replica_tag),
            ),
            None => (None, None),
        };

        let opts = InnerOptions {
            replicas,
            primary_replica_tag,
            primary_key: self.primary_key.as_ref(),
            durability: self.durability,
            shards: self.shards,
        };

        opts.serialize(serializer)
    }
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct IndexCreateOptions {
    pub multi: Option<bool>,
    pub geo: Option<bool>,
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct IndexRenameOptions {
    pub overwrite: Option<bool>,
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct InsertOptions {
    /// possible values are hard and soft. This option will override the table or query’s durability setting (set in run).
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving and caching it,
    /// but before the write has been committed to disk.
    pub durability: Option<Durability>,
    /// - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made,
    ///   only including the documents actually updated.
    /// - `false`: do not return a `changes` array (the default).
    /// - `"always"`: behave as `true`, but include all documents the command tried to update whether
    ///   or not the update was successful. (This was the behavior of true pre-2.0.)
    pub return_changes: Option<ReturnChanges>,
    /// Determine handling of inserting documents with the same primary key as existing entries.
    /// There are three built-in methods: `"error"`, `"replace"` or `"update"`; alternatively, you may provide a conflict resolution function
    /// - `"error"`: Do not insert the new document and record the conflict as an error. This is the default.
    /// - `"replace"`: Replace the old document in its entirety with the new one.
    /// - `"update"`: Update fields of the old document with fields from the new one.
    /// - `function (id, oldDoc, newDoc) { return resolvedDoc }`: a function that receives the id, old and new documents as arguments and returns a document which will be inserted in place of the conflicted one.
    pub conflict: Option<Conflict>,
    /// If true, and if the user has the config permission, ignores any write hook, inserting the document unmodified
    pub ignore_write_hook: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Conflict {
    Replace,
    Update,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ReturnChanges {
    Bool(bool),
    Always,
}

impl Serialize for ReturnChanges {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Always => "always".serialize(serializer),
        }
    }
}

impl From<bool> for ReturnChanges {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct UpdateOptions {
    /// possible values are hard and soft. This option will override the table or query’s durability setting (set in run).
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving and caching it,
    /// but before the write has been committed to disk.
    pub durability: Option<Durability>,
    /// - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made,
    ///   only including the documents actually updated.
    /// - `false`: do not return a `changes` array (the default).
    /// - `"always"`: behave as `true`, but include all documents the command tried to update whether
    ///   or not the update was successful. (This was the behavior of true pre-2.0.)
    pub return_changes: Option<ReturnChanges>,
    /// if set to `true`, executes the update and distributes the result to replicas
    /// in a non-atomic fashion. This flag is required to perform non-deterministic updates,
    /// such as those that require reading data from another table.
    pub non_atomic: Option<bool>,
    /// If `true`, and if the user has the config permission,
    /// ignores any write hook when performing the update.
    pub ignore_write_hook: Option<bool>,
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct ReplaceOptions {
    /// possible values are hard and soft. This option will override the table or query’s durability setting (set in run).
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving and caching it,
    /// but before the write has been committed to disk.
    pub durability: Option<Durability>,
    /// - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made,
    ///   only including the documents actually updated.
    /// - `false`: do not return a `changes` array (the default).
    /// - `"always"`: behave as `true`, but include all documents the command tried to update whether
    ///   or not the update was successful. (This was the behavior of true pre-2.0.)
    pub return_changes: Option<ReturnChanges>,
    /// if set to `true`, executes the update and distributes the result to replicas
    /// in a non-atomic fashion. This flag is required to perform non-deterministic updates,
    /// such as those that require reading data from another table.
    pub non_atomic: Option<bool>,
    /// If `true`, and if the user has the config permission,
    /// ignores any write hook when performing the update.
    pub ignore_write_hook: Option<bool>,
}

#[skip_serializing_none]
#[derive(
    Debug, Clone, Copy, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder,
)]
pub struct DeleteOptions {
    /// possible values are hard and soft. This option will override the table or query’s durability setting (set in run).
    /// In soft durability mode RethinkDB will acknowledge the write immediately after receiving and caching it,
    /// but before the write has been committed to disk.
    pub durability: Option<Durability>,
    /// - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made,
    ///   only including the documents actually updated.
    /// - `false`: do not return a `changes` array (the default).
    /// - `"always"`: behave as `true`, but include all documents the command tried to update whether
    ///   or not the update was successful. (This was the behavior of true pre-2.0.)
    pub return_changes: Option<ReturnChanges>,
    /// If `true`, and if the user has the config permission,
    /// ignores any write hook when performing the update.
    pub ignore_write_hook: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder)]
pub struct BetweenOptions {
    /// If `index` is set to the name of a secondary index, `between` will return all
    /// documents where that index’s value is in the specified range (it uses the primary
    /// key by default).
    pub index: Option<String>,
    /// `left_bound` may be set to open or closed to indicate whether or not to include
    /// that endpoint of the range (*by default*, `left_bound` is `closed`).
    pub left_bound: Option<Status>,
    /// `right_bound` may be set to open or closed to indicate whether or not to include
    /// that endpoint of the range (*by default*, `right_bound` is `open`).
    pub right_bound: Option<Status>,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Open,
    Closed,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder)]
pub struct DuringOptions {
    /// `left_bound` may be set to open or closed to indicate whether or not to include
    /// that endpoint of the range (*by default*, `left_bound` is `closed`).
    pub left_bound: Option<Status>,
    /// `right_bound` may be set to open or closed to indicate whether or not to include
    /// that endpoint of the range (*by default*, `right_bound` is `open`).
    pub right_bound: Option<Status>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder)]
pub struct SliceOptions {
    /// `left_bound` may be set to open or closed to indicate whether or not to include
    /// that endpoint of the range (*by default*, `left_bound` is `closed`).
    pub left_bound: Option<Status>,
    /// `right_bound` may be set to open or closed to indicate whether or not to include
    /// that endpoint of the range (*by default*, `right_bound` is `open`).
    pub right_bound: Option<Status>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct UnionOptions {
    pub interleave: Option<Interleave>,
}

#[derive(Debug, Clone)]
pub enum Interleave {
    Bool(bool),
    Command(Command),
}

impl Serialize for Interleave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Command(cmd) => cmd.serialize(serializer),
        }
    }
}

impl From<bool> for Interleave {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for Interleave {
    fn from(value: String) -> Self {
        Self::Command(Command::from_json(value))
    }
}
impl From<&str> for Interleave {
    fn from(value: &str) -> Self {
        Self::Command(Command::from_json(value))
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd, WithOpts, OptionsBuilder)]
pub struct GroupOptions {
    pub index: Option<String>,
    pub multi: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct FoldOptions {
    pub emit: Option<Command>,
    pub final_emit: Option<Command>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct JsOptions {
    pub timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct HttpOptions {
    /// timeout period in seconds to wait before aborting the connect (default `30`)
    pub timeout: Option<f64>,
    /// number of retry attempts to make after failed connections (default `5`)
    pub attempts: Option<i64>,
    /// number of redirect and location headers to follow (default `1`)
    pub redirects: Option<i64>,
    /// if `true`, verify the server’s SSL certificate (default `true`)
    pub verify: Option<bool>,
    /// string specifying the format to return results in
    pub result_format: Option<ResultFormat>,

    /// HTTP method to use for the request. Default: `GET`
    pub method: Option<HttpMethod>,
    /// object giving authentication
    pub auth: Option<HttpAuth>,
    /// object specifying URL parameters to append to the URL as encoded key/value pairs. { query: 'banana', limit: 2 } will be appended as ?query=banana&limit=2. Default: no parameters.
    pub params: Option<serde_json::Value>,
    /// Extra header lines to include. The value may be an array of strings or an object. Default: Accept-Encoding: deflate;q=1, gzip;q=0.5 and User-Agent: RethinkDB/<VERSION>.
    pub header: Option<serde_json::Value>,
    /// Data to send to the server on a POST, PUT, PATCH, or DELETE request. For POST requests, data may be either an object (which will be written to the body as form-encoded key/value pairs) or a string; for all other requests, data will be serialized as JSON and placed in the request body, sent as Content-Type: application/json. Default: no data will be sent.
    pub data: Option<serde_json::Value>,

    /// This option may specify either a built-in pagination strategy (see below), or a function to provide the next URL and/or params to request.
    pub page: Option<i64>,
    /// An integer specifying the maximum number of requests to issue using the page functionality. This is to prevent overuse of API quotas, and must be specified with page.
    /// - `-1`: no limit
    /// - `0`: no requests will be made, an empty stream will be returned
    /// - `n`: `n` requests will be made
    pub page_limit: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ResultFormat {
    /// always return a string
    Text,
    /// parse the result as JSON, raising an error on failure
    Json,
    /// parse the result as Padded JSON
    Jsonp,
    /// return a binary object
    Binary,
    /// parse the result based on its Content-Type (the default):
    /// - `application/json`: as `json`
    /// - `application/json-p`, `text/json-p`, `text/javascript`: as `jsonp`
    /// - `audio/*`, `video/*`, `image/*`, `application/octet-stream`: as `binary`
    /// - anything else: as `text`
    Auto,
}

#[derive(Debug, Clone, Copy, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct HttpAuth {
    /// basic (default) or digest
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// username
    pub user: Option<String>,
    /// password in plain text
    pub pass: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct CircleOptions {
    /// the number of vertices in the polygon or line. Defaults to 32.
    pub num_vertices: Option<i64>,
    /// the reference ellipsoid to use for geographic coordinates. Possible values are WGS84 (the default), a common standard for Earth’s geometry, or unit_sphere, a perfect sphere of 1 meter radius.
    pub geo_system: Option<String>,
    /// Unit for the radius distance. Possible values are m (meter, the default), km (kilometer), mi (international mile), nm (nautical mile), ft (international foot).
    pub unit: Option<String>,
    /// if `true` (the default) the circle is filled, creating a polygon; if false the circle is unfilled (creating a line).
    pub fill: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct DistanceOptions {
    pub geo_system: Option<String>,
    pub unit: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct GetNearestOptions {
    pub index: Option<String>,
    pub max_results: Option<i64>,
    pub unit: Option<String>,
    pub max_dist: Option<i64>,
    pub geo_system: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct GrantOptions {
    pub read: Option<GrantValue>,
    pub write: Option<GrantValue>,
    pub connect: Option<GrantValue>,
    pub config: Option<GrantValue>,
}

#[derive(Debug, Copy, Clone)]
pub enum GrantValue {
    Bool(bool),
    Null,
}

impl From<bool> for GrantValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl Serialize for GrantValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::Null => serializer.serialize_none(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct WaitOptions {
    pub wait_for: Option<WaitFor>,
    pub timeout: Option<i64>,
}

#[derive(Debug, Copy, Clone, Default, Serialize)]
pub enum WaitFor {
    ReadyForOutdatedReads,
    ReadyForReads,
    ReadyForWrites,
    #[default]
    AllReplicasReady,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default, WithOpts, OptionsBuilder)]
pub struct ReconfigureOptions {
    pub shards: Option<i64>,
    pub replicas: Option<i64>,
    pub primary_replica_tag: Option<String>,
    pub dry_run: Option<bool>,
    pub nonvoting_replica_tags: Option<serde_json::Value>,
    pub emergency_repair: Option<String>,
}
