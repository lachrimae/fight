use crate::app::{App, Config};
lazy_static::lazy_static! {
    pub static ref CFG: Config = Config::from_env().unwrap();
    pub static ref APP: App = App::from_cfg(&CFG).unwrap();
}
