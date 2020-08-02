use clap::{ArgMatches};
use super::SubCommandDispatcher;
use crate::list;

pub struct Dispatcher {}

impl SubCommandDispatcher for Dispatcher {
  fn dispatch(matches: &ArgMatches) {
    Some(matches);

    let lists = list::get_lists(".").expect("Cannot read directory");
    println!("{:?}", lists);
  }
}
