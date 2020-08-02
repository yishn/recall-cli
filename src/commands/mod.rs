use clap::ArgMatches;

mod info;
mod learn;
mod list;
mod study;

pub use info::Dispatcher as InfoDispatcher;
pub use learn::Dispatcher as LearnDispatcher;
pub use list::Dispatcher as ListDispatcher;
pub use study::Dispatcher as StudyDispatcher;

pub trait SubCommandDispatcher {
  fn dispatch(matches: &ArgMatches);
}
