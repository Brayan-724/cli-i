#[macro_export]
macro_rules! flush {
    ($stdout:ident) => (
        $stdout.flush().unwrap();
    )
}

#[macro_export]
macro_rules! clear {
    ($stdout:ident) => (
        write!($stdout, "{}", termion::clear::All)
    )
}

#[macro_export]
macro_rules! clear_all {
    ($stdout:ident) => {
        match write!($stdout, "{}", termion::cursor::Goto(1, 1)) {
            Err(e) => Result::Err(e),
            _ => write!($stdout, "{}", termion::clear::All)
        }
    }
}

#[macro_export]
macro_rules! cursor_goto {
    ($stdout: ident; $x: expr, $y: expr) => (
        write!($stdout, "{}", termion::cursor::Goto($x, $y))
    )
}

#[macro_export]
macro_rules! cursor_left {
    ($stdout: ident; $count: expr) => (
        write!($stdout, "{}", termion::cursor::Left($count))
    )
}

#[macro_export]
macro_rules! cursor_start {
    ($stdout: ident) => (
        cursor_left!($stdout; 999)
    )
}
