use serde::Serialize;

use crate::{cmd::options::Index, Command};

use super::{Args, ArgsWithOpt, WithOpts};

/// Required first argument and optional second
///
/// Variants:
///
/// ```rust,ignore
/// // first required argument
/// r.example(arg);
///
/// // with both arguments as array of two elements
/// r.example(r.args([arg1, arg2]));
///
/// // with both arguments as tuple of two elements
/// r.example(r.args((arg1, arg2)));
///
/// // first required argument with command options
/// r.example(r.with_opt(arg, opt));
///
/// // with both arguments as array of two elements with command options
/// r.example(r.with_opt(r.args([arg1, arg2]), opt));
///
/// // with both arguments as tuple of two elements with command options
/// r.example(r.with_opt(r.args((arg1, arg2)), opt));
/// ```
pub trait OneAndSecondOptionalArg<P> {
    fn with_cmd(self, cmd: Command) -> Command;
}

impl<T, P> OneAndSecondOptionalArg<P> for T
where
    T: Serialize,
{
    fn with_cmd(self, cmd: Command) -> Command {
        cmd.with_arg(Command::from_json(self))
    }
}

impl<T, P> OneAndSecondOptionalArg<P> for Args<[T; 2]>
where
    T: Serialize,
{
    fn with_cmd(self, cmd: Command) -> Command {
        self.0
            .iter()
            .fold(cmd, |cmd, arg| cmd.with_arg(Command::from_json(arg)))
    }
}

impl<T1, T2, P> OneAndSecondOptionalArg<P> for Args<(T1, T2)>
where
    T1: Serialize,
    T2: Serialize,
{
    fn with_cmd(self, cmd: Command) -> Command {
        cmd.with_arg(Command::from_json(self.0 .0))
            .with_arg(Command::from_json(self.0 .1))
    }
}

impl<T, P> OneAndSecondOptionalArg<P> for ArgsWithOpt<T, P>
where
    T: Serialize,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = cmd.with_arg(Command::from_json(self.0));
        self.1.with_opts(cmd)
    }
}

impl<T1, T2, P> OneAndSecondOptionalArg<P> for ArgsWithOpt<Args<(T1, T2)>, P>
where
    T1: Serialize,
    T2: Serialize,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = cmd
            .with_arg(Command::from_json(self.0 .0 .0))
            .with_arg(Command::from_json(self.0 .0 .1));
        self.1.with_opts(cmd)
    }
}

impl<T, P> OneAndSecondOptionalArg<P> for ArgsWithOpt<Args<[T; 2]>, P>
where
    T: Serialize,
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        let cmd = self
            .0
             .0
            .iter()
            .fold(cmd, |cmd, arg| cmd.with_arg(Command::from_json(arg)));
        self.1.with_opts(cmd)
    }
}

impl OneAndSecondOptionalArg<Index> for Index {
    fn with_cmd(self, cmd: Command) -> Command {
        self.with_opts(cmd)
    }
}
