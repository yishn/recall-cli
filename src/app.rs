pub fn name() -> &'static str {
  env!("CARGO_PKG_NAME")
}

pub fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}

pub fn author() -> &'static str {
  env!("CARGO_PKG_AUTHORS")
}

pub fn description() -> &'static str {
  env!("CARGO_PKG_DESCRIPTION")
}
