use termion::raw::RawTerminal;
use crate::component::Component;

#[derive(Debug)]
pub struct Screen {
    pub is_static: bool,
    pub auto_clear: bool,
    pub components: Box<[&'static dyn Component]>
}

impl Screen {
    fn start(stdout: &mut RawTerminal<std::io::Stdout>) {
    }
}
