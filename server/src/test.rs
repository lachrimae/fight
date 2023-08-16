use crate::app::{App, Config};
use futures::executor;
lazy_static::lazy_static! {
    pub static ref CFG: Config = Config::from_env().unwrap();
    pub static ref APP: App = executor::block_on(App::from_cfg(&CFG)).unwrap();
}
