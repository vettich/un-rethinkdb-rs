use crate::Command;

use super::{ArgsWithOpt, WithOpts};

pub trait Opt<P> {
    fn with_cmd(self, cmd: Command) -> Command;
}

impl<P> Opt<P> for P
where
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        self.with_opts(cmd)
    }
}

impl<P> Opt<P> for () {
    fn with_cmd(self, cmd: Command) -> Command {
        cmd
    }
}

impl<P> Opt<P> for ArgsWithOpt<(), P>
where
    P: WithOpts,
{
    fn with_cmd(self, cmd: Command) -> Command {
        self.1.with_opts(cmd)
    }
}
