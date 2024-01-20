use serde::Serialize;

use crate::{cmd::options::Index, Command};

use super::{ArgsWithOpt, WithOpts};

pub trait Arg<P> {
    fn with_cmd(self, cmd: Command) -> Command;
}

impl<T, P> Arg<P> for T
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

impl<T, P> Arg<P> for ArgsWithOpt<T, P>
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

impl Arg<Index> for Index {
    fn with_cmd(self, cmd: Command) -> Command {
        self.with_opts(cmd)
    }
}
