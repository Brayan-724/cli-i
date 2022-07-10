#![feature(io_error_uncategorized)]
#![feature(trace_macros)]

#[macro_use]
extern crate clii;

use std::io::{ stdin, stdout, Write };
use termion::event::{ Key, Event };
use termion::input::{ TermRead, MouseTerminal };
use termion::raw::IntoRawMode;
use colored::*;
use clii::component::*;
use clii::app::*;
use clii::screen::*;

#[allow(unreachable_code)]
fn main() {
    let _stdin = stdin();

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let text_component = create_box!(! ;
        paragraph Color::Green, {
            text "Hol", Color::Red; 
            text "a que no hace. ", Color::Green;
            text "Otro texto bonito: ";
            text "Mi valor", Color::Red, Styles::Italic;
        };

        text "Hola";
    );

    let screen01 = ScreenBuilder::new()
        .set_auto_clear(true)
        .add_component(text_component)
        .build();

    write!(stdout, "{:?}", screen01).unwrap();

    flush!(stdout);

    return ();
    for c in _stdin.events() {
        let ch = c.unwrap();
        write!(stdout, "Pressed: {:?}\n", &ch).unwrap();
        cursor_start!(stdout).unwrap();
        flush!(stdout);

        match ch {
            Event::Key(Key::Ctrl('q')) => break,
            _ => ()
        }

        flush!(stdout);
    }
}
