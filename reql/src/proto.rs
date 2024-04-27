use crate::cmd::run::{Db, Options};
use crate::{err, r, Func};
use ql2::query::QueryType;
use ql2::term::TermType;
use serde::ser::{self, Serialize, Serializer};
use serde_json::value::{Number, Value};
use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};
use std::{fmt, str};

#[derive(Debug, Clone)]
pub enum Datum {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Datum>),
    Object(HashMap<String, Datum>),
    Value(Value),
    Command(Box<Command>),
}

impl Datum {
    pub fn into_command(self) -> Command {
        self.into()
    }

    fn has_implicit_var_arg(&self) -> bool {
        match self {
            Datum::Command(cmd) => cmd.has_implicit_var_arg(),
            Datum::Object(obj) => obj.iter().any(|(_, datum)| datum.has_implicit_var_arg()),
            _ => false,
        }
    }
}

impl Default for Datum {
    fn default() -> Self {
        Self::Null
    }
}

impl Serialize for Datum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(boolean) => boolean.serialize(serializer),
            Self::Number(num) => num.serialize(serializer),
            Self::String(string) => string.serialize(serializer),
            Self::Array(arr) => (TermType::MakeArray as i32, arr).serialize(serializer),
            Self::Object(map) => map.serialize(serializer),
            Self::Value(value) => value.serialize(serializer),
            Self::Command(cmd) => cmd.serialize(serializer),
        }
    }
}

#[allow(array_into_iter)]
#[allow(clippy::into_iter_on_ref)]
impl<const N: usize> From<[Command; N]> for Command {
    fn from(arr: [Command; N]) -> Self {
        let mut query = Self::new(TermType::MakeArray);
        for arg in arr.into_iter() {
            query = query.with_arg(arg);
        }
        query
    }
}

impl From<Value> for Datum {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Bool(boolean) => Self::Bool(boolean),
            Value::Number(num) => Self::Number(num),
            Value::String(string) => Self::String(string),
            Value::Array(arr) => Self::Array(arr.into_iter().map(Into::into).collect()),
            Value::Object(map) => Self::Object(
                map.into_iter()
                    .map(|(key, value)| (key, value.into()))
                    .collect(),
            ),
        }
    }
}

/// The query that will be sent to RethinkDB
#[derive(Debug, Clone)]
pub enum Command {
    Boxed(Box<Command>),
    Data {
        typ: TermType,
        datum: Option<super::Result<Datum>>,
        args: VecDeque<Command>,
        opts: Option<super::Result<Datum>>,
        change_feed: bool,
    },
}

impl Command {
    #[doc(hidden)]
    pub fn new(typ: TermType) -> Self {
        Self::Data {
            typ,
            datum: None,
            args: VecDeque::new(),
            opts: None,
            change_feed: false,
        }
    }

    #[doc(hidden)]
    pub fn var(id: u64) -> Self {
        let index = Self::from_json(id);
        Self::new(TermType::Var).with_arg(index)
    }

    #[doc(hidden)]
    pub fn with_parent(mut self, parent: Command) -> Self {
        self.set_change_feed(self.change_feed() || parent.change_feed());
        self.mut_args().push_front(parent);
        self
    }

    #[doc(hidden)]
    pub fn with_arg<T>(mut self, arg: T) -> Self
    where
        T: Into<Command>,
    {
        let arg = arg.into();
        self.mut_args().push_back(arg);
        self
    }

    #[doc(hidden)]
    pub fn with_opts<T>(mut self, opts: T) -> Self
    where
        T: Serialize + 'static,
    {
        // retrieve Datum from Command if possible
        let opts = match Self::from_json_2(opts) {
            Self::Data {
                typ: TermType::Datum,
                datum: Some(datum),
                ..
            } => datum,
            Self::Boxed(cmd) => match *cmd {
                Self::Data {
                    typ: TermType::Datum,
                    datum: Some(datum),
                    ..
                } => datum,
                cmd => Ok(Datum::Command(Box::new(cmd))),
            },
            cmd => Ok(Datum::Command(Box::new(cmd))),
        };

        self.set_opts(opts);
        self
    }

    #[doc(hidden)]
    pub fn from_json<T>(arg: T) -> Self
    where
        T: Serialize,
    {
        serde_json::to_value(arg).map_err(super::Error::from).into()
    }

    #[doc(hidden)]
    pub fn from_json_2<T>(arg: T) -> Self
    where
        T: Serialize + Any,
    {
        if arg.type_id() == TypeId::of::<Command>() {
            let cmd: Box<dyn Any> = Box::new(arg);
            match cmd.downcast::<Command>() {
                Ok(cmd) => Command::Boxed(cmd),
                Err(_) => {
                    let mut cmd = Command::new(TermType::Datum);
                    cmd.set_datum(Err(super::Error::Driver(super::Driver::Other(
                        "cannot downcast to Command".into(),
                    ))));
                    cmd
                }
            }
        } else {
            serde_json::to_value(arg).map_err(super::Error::from).into()
        }
    }

    pub(crate) fn is_null_json(&self) -> bool {
        if self.typ() != TermType::Datum {
            return false;
        }

        if let Some(Ok(Datum::Null)) = self.datum() {
            return true;
        }

        false
    }

    pub(crate) fn wrap_by_func(self) -> Self {
        if !self.has_implicit_var_arg() {
            return self;
        }

        Func::new(vec![1], self).into_cmd()
    }

    fn has_implicit_var_arg(&self) -> bool {
        match self {
            Self::Boxed(cmd) => cmd.has_implicit_var_arg(),
            Self::Data {
                typ, args, datum, ..
            } => {
                if *typ == TermType::ImplicitVar {
                    return true;
                }

                if *typ == TermType::Func {
                    return false;
                }

                if args.iter().any(|cmd| cmd.has_implicit_var_arg()) {
                    return true;
                }

                if let Some(Ok(datum)) = datum {
                    return datum.has_implicit_var_arg();
                }

                false
            }
        }
    }

    pub(crate) fn typ(&self) -> TermType {
        match self {
            Self::Boxed(cmd) => cmd.typ(),
            Self::Data { typ, .. } => *typ,
        }
    }

    pub(crate) fn change_feed(&self) -> bool {
        match self {
            Self::Boxed(cmd) => cmd.change_feed(),
            Self::Data { change_feed, .. } => *change_feed,
        }
    }

    pub(crate) fn mark_change_feed(mut self) -> Self {
        self.set_change_feed(true);
        self
    }

    fn set_change_feed(&mut self, v: bool) {
        match self {
            Self::Boxed(cmd) => {
                cmd.set_change_feed(v);
            }
            Self::Data { change_feed, .. } => *change_feed = v,
        }
    }

    fn datum(&self) -> &Option<super::Result<Datum>> {
        match self {
            Self::Boxed(cmd) => cmd.datum(),
            Self::Data { datum, .. } => datum,
        }
    }

    fn set_datum(&mut self, new_datum: super::Result<Datum>) {
        match self {
            Self::Boxed(cmd) => cmd.set_datum(new_datum),
            Self::Data { datum, .. } => *datum = Some(new_datum),
        }
    }

    fn set_opts(&mut self, new_opts: super::Result<Datum>) {
        match self {
            Self::Boxed(cmd) => cmd.set_opts(new_opts),
            Self::Data { opts, .. } => *opts = Some(new_opts),
        }
    }

    fn mut_args(&mut self) -> &mut VecDeque<Command> {
        match self {
            Self::Boxed(cmd) => cmd.mut_args(),
            Self::Data { args, .. } => args,
        }
    }
}

impl From<Datum> for Command {
    fn from(datum: Datum) -> Self {
        Ok(datum).into()
    }
}

impl From<super::Result<Datum>> for Command {
    fn from(result: super::Result<Datum>) -> Self {
        let mut query = Self::new(TermType::Datum);
        query.set_datum(result);
        query
    }
}

#[doc(hidden)]
impl From<Value> for Command {
    fn from(value: Value) -> Self {
        Datum::from(value).into()
    }
}

#[doc(hidden)]
impl From<super::Result<Value>> for Command {
    fn from(result: super::Result<Value>) -> Self {
        match result {
            Ok(value) => Datum::from(value).into(),
            Err(error) => (Err(error) as super::Result<Datum>).into(),
        }
    }
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Boxed(cmd) => cmd.serialize(serializer),
            Self::Data {
                typ,
                datum,
                args,
                opts,
                ..
            } => match typ {
                TermType::Datum => match &datum {
                    Some(Ok(datum)) => datum.serialize(serializer),
                    Some(Err(error)) => Err(ser::Error::custom(error)),
                    _ => (None as Option<Datum>).serialize(serializer),
                },
                _ => {
                    let typ = *typ as i32;
                    match &opts {
                        Some(Ok(map)) => (typ, args, map).serialize(serializer),
                        None => {
                            if args.is_empty() {
                                [typ].serialize(serializer)
                            } else {
                                (typ, args).serialize(serializer)
                            }
                        }
                        Some(Err(error)) => Err(ser::Error::custom(error)),
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Payload<'a>(
    pub(crate) QueryType,
    pub(crate) Option<&'a Command>,
    pub(crate) Options,
);

impl Serialize for Payload<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Payload(typ, qry, opts) = self;
        let typ = *typ as i32;
        match qry {
            Some(query) => (typ, query, opts).serialize(serializer),
            None => (typ,).serialize(serializer),
        }
    }
}

impl Payload<'_> {
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, err::Error> {
        Ok(serde_json::to_vec(self)?)
    }
}

// for debugging purposes only
impl fmt::Display for Payload<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the serialised string if we can
        if let Ok(payload) = self.to_bytes() {
            if let Ok(payload) = str::from_utf8(&payload) {
                return write!(f, "{}", payload);
            }
        }
        // otherwise just print the debug form
        write!(f, "{:?}", self)
    }
}

impl Serialize for Db {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Self(name) = self;
        let cmd = r.db(name.clone());
        cmd.serialize(serializer)
    }
}
