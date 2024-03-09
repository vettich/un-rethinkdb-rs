use crate::Command;

mod arg;
mod do_args;
mod many;
mod one_two_opt;
mod opt;

pub use arg::Arg;
pub use do_args::DoArgs;
pub use many::ManyArgs;
pub use one_two_opt::OneAndSecondOptionalArg;
pub use opt::Opt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Args<T>(pub(crate) T);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArgsWithOpt<T, P>(pub(crate) T, pub(crate) P);

pub trait WithOpts {
    fn with_opts(self, cmd: Command) -> Command;
}

impl WithOpts for Command {
    fn with_opts(self, cmd: Command) -> Command {
        cmd.with_opts(self)
    }
}
