use ql2::term::TermType;
use serde::Serialize;

use crate::{cmd::options::Index, r, Command};

use super::{Args, ArgsWithOpt, WithOpts};

pub trait ManyArgs<P> {
    fn with_cmd(self, cmd: Command) -> Command;
}

impl<T, P> ManyArgs<P> for T
where
    T: Serialize + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let arg = Command::from_json_2(self);
        if arg.is_null_json() {
            // if argument is `null` or `()` then skip to add it
            cmd
        } else {
            cmd.with_arg(arg.wrap_by_func())
        }
    }
}

impl<T, P> ManyArgs<P> for Args<&[T]>
where
    T: Serialize + Clone + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        self.0.iter().cloned().fold(cmd, |cmd, arg| {
            cmd.with_arg(Command::from_json_2(arg).wrap_by_func())
        })
    }
}

impl<T, P> ManyArgs<P> for Args<&Vec<T>>
where
    T: Serialize + Clone + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        ManyArgs::<P>::with_cmd(r.args(self.0.as_slice()), cmd)
    }
}

impl<T, P, const N: usize> ManyArgs<P> for Args<[T; N]>
where
    T: Serialize + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        self.0.into_iter().fold(cmd, |cmd, arg| {
            cmd.with_arg(Command::from_json_2(arg).wrap_by_func())
        })
    }
}

impl<T, P> ManyArgs<P> for Args<Vec<T>>
where
    T: Serialize + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        self.0.into_iter().fold(cmd, |cmd, arg| {
            cmd.with_arg(Command::from_json_2(arg).wrap_by_func())
        })
    }
}

impl<P> ManyArgs<P> for Args<Command> {
    fn with_cmd(self, cmd: Command) -> Command {
        cmd.with_arg(Command::new(TermType::Args).with_arg(self.0.wrap_by_func()))
    }
}

impl<T1, T2, P> ManyArgs<P> for Args<(T1, T2)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .1).wrap_by_func())
    }
}

impl<T1, T2, T3, P> ManyArgs<P> for Args<(T1, T2, T3)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .1).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .2).wrap_by_func())
    }
}

impl<T1, T2, T3, T4, P> ManyArgs<P> for Args<(T1, T2, T3, T4)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
    T4: Serialize + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .1).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .2).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .3).wrap_by_func())
    }
}

impl<T1, T2, T3, T4, T5, P> ManyArgs<P> for Args<(T1, T2, T3, T4, T5)>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
    T4: Serialize + 'static,
    T5: Serialize + 'static,
{
    fn with_cmd(self, cmd: Command) -> Command {
        cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .1).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .2).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .3).wrap_by_func())
            .with_arg(Command::from_json_2(self.0 .4).wrap_by_func())
    }
}

impl<T, P> ManyArgs<P> for ArgsWithOpt<T, P>
where
    T: Serialize + 'static,
    P: WithOpts,
{
    fn with_cmd(self, mut cmd: Command) -> Command {
        // if argument is `null` or `()` then skip to add it
        let arg = Command::from_json_2(self.0);
        if !arg.is_null_json() {
            cmd = cmd.with_arg(arg.wrap_by_func())
        };

        self.1.with_opts(cmd)
    }
}

impl<T, P> ManyArgs<P> for ArgsWithOpt<Args<&[T]>, P>
where
    T: Serialize + Clone + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = self.0 .0.iter().cloned().fold(cmd, |cmd, arg| {
            cmd.with_arg(Command::from_json_2(arg).wrap_by_func())
        });
        self.1.with_opts(cmd)
    }
}

impl<T, P, const N: usize> ManyArgs<P> for ArgsWithOpt<Args<[T; N]>, P>
where
    T: Serialize + Clone + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        ManyArgs::<P>::with_cmd(r.with_opt(r.args(self.0 .0.as_slice()), self.1), cmd)
    }
}

impl<T, P> ManyArgs<P> for ArgsWithOpt<Args<&Vec<T>>, P>
where
    T: Serialize + Clone + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        ManyArgs::<P>::with_cmd(r.with_opt(r.args(self.0 .0.as_slice()), self.1), cmd)
    }
}

impl<T, P> ManyArgs<P> for ArgsWithOpt<Args<Vec<T>>, P>
where
    T: Serialize + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = self.0 .0.into_iter().fold(cmd, |cmd, arg| {
            cmd.with_arg(Command::from_json_2(arg).wrap_by_func())
        });
        self.1.with_opts(cmd)
    }
}

impl<T, P> ManyArgs<P> for ArgsWithOpt<Args<(T,)>, P>
where
    T: Serialize + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = cmd.with_arg(Command::from_json_2(self.0 .0).wrap_by_func());
        self.1.with_opts(cmd)
    }
}

impl<T1, T2, P> ManyArgs<P> for ArgsWithOpt<Args<(T1, T2)>, P>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = ManyArgs::<P>::with_cmd(self.0, cmd);
        self.1.with_opts(cmd)
    }
}

impl<T1, T2, T3, P> ManyArgs<P> for ArgsWithOpt<Args<(T1, T2, T3)>, P>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = ManyArgs::<P>::with_cmd(self.0, cmd);
        self.1.with_opts(cmd)
    }
}

impl<T1, T2, T3, T4, P> ManyArgs<P> for ArgsWithOpt<Args<(T1, T2, T3, T4)>, P>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
    T4: Serialize + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = ManyArgs::<P>::with_cmd(self.0, cmd);
        self.1.with_opts(cmd)
    }
}

impl<T1, T2, T3, T4, T5, P> ManyArgs<P> for ArgsWithOpt<Args<(T1, T2, T3, T4, T5)>, P>
where
    T1: Serialize + 'static,
    T2: Serialize + 'static,
    T3: Serialize + 'static,
    T4: Serialize + 'static,
    T5: Serialize + 'static,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = ManyArgs::<P>::with_cmd(self.0, cmd);
        self.1.with_opts(cmd)
    }
}

impl<P> ManyArgs<P> for ArgsWithOpt<Args<Command>, P>
where
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = cmd.with_arg(Command::new(TermType::Args).with_arg(self.0 .0.wrap_by_func()));
        self.1.with_opts(cmd)
    }
}

impl<P> ManyArgs<P> for Index {
    fn with_cmd(self, cmd: Command) -> Command {
        self.with_opts(cmd)
    }
}
