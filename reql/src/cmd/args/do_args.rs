use ql2::term::TermType;
use serde::Serialize;
use serde_json::Value;

use crate::Command;

use super::Args;

pub trait DoArgs {
    fn build(self, parent: Option<Command>) -> Command;
}

impl<T> DoArgs for T
where
    T: Serialize + 'static,
{
    fn build(self, parent: Option<Command>) -> Command {
        let arg = Command::from_json_2(self).wrap_by_func();
        let cmd = Command::new(TermType::Funcall).with_arg(arg);
        match parent {
            Some(p) => cmd.with_arg(p),
            None => cmd,
        }
    }
}

impl<T> DoArgs for Args<&[T]>
where
    T: Serialize + Clone + 'static,
{
    fn build(self, parent: Option<Command>) -> Command {
        let args = self.0;

        let Some((fn_arg, args)) = args.split_last() else {
            return Value::Null.build(parent);
        };

        let cmd = fn_arg.clone().build(parent);

        args.iter().cloned().fold(cmd, |cmd, arg| {
            cmd.with_arg(Command::from_json_2(arg).wrap_by_func())
        })
    }
}

impl<T1, T2> DoArgs for Args<(T1, T2)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
{
    fn build(self, parent: Option<Command>) -> Command {
        let cmd = self.0 .1.build(parent);
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
    }
}

impl<T1, T2, T3> DoArgs for Args<(T1, T2, T3)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
{
    fn build(self, parent: Option<Command>) -> Command {
        let cmd = self.0 .2.build(parent);
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .1).wrap_by_func())
    }
}

impl<T1, T2, T3, T4> DoArgs for Args<(T1, T2, T3, T4)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
    T4: Serialize + 'static,
{
    fn build(self, parent: Option<Command>) -> Command {
        let cmd = self.0 .3.build(parent);
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .1).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .2).wrap_by_func())
    }
}

impl<T1, T2, T3, T4, T5> DoArgs for Args<(T1, T2, T3, T4, T5)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
    T4: Serialize + 'static,
    T5: Serialize + 'static,
{
    fn build(self, parent: Option<Command>) -> Command {
        let cmd = self.0 .4.build(parent);
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .1).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .2).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .3).wrap_by_func())
    }
}
