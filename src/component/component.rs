#[warn(unreachable_patterns)]

use std::io::{ self, Write };
use colored::*;

#[doc = "Different types of component"]
#[derive(Debug, Clone)]
pub enum ComponentType {
    Text(String),
    Box(Box<[Component]>),
    Custom(Box<>)
}

#[doc = "Can be draw it"]
pub trait Drawable {
    fn draw_<W: Write>(&self, stdout: Box<&mut W>) -> io::Result<usize> where Self: Sized;
    fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<usize> where Self: Sized;
}

pub trait Controllable {
    fn start(&self) -> Result<(), String> {
        Ok(())
    }
}


#[doc = "A component with it style"]
#[derive(Debug, Clone)]
pub struct Component {
    component_type: ComponentType,
    is_static: bool,
    color: Color,
    style: Styles
}

pub trait CustomComponent {}


impl dyn Drawable  {
    fn draw_<W: Write>(&self, stdout: Box<&mut W>) -> io::Result<usize> where Self: Sized {
        Drawable::draw_(&(*self), stdout)
    }

    fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<usize> {
         (*self).draw(&mut stdout)
    }
} 

impl dyn Controllable  {}

fn apply_style<T>(target: T, style: Styles) -> Result<ColoredString, &'static str> where T: Colorize + Clone {
    match style {
        Styles::Clear => Ok(target.clone().clear()),
        Styles::Bold => Ok(target.clone().bold()),
        Styles::Italic => Ok(target.clone().italic()),
        _ => Err("Style not implemented")
    }
}

fn normalize_text(text: &String) -> String {
    let cursor_left = termion::cursor::Left(999);
    let cursor_left = format!("\n{}", cursor_left);

    return text.replace("\n", cursor_left.as_str());
}

impl Drawable for Component {
    fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<usize> {
        self.draw_(Box::new(stdout))
    }

    fn draw_<W: Write>(&self, stdout: Box<&mut W>) -> io::Result<usize> where Self: Sized {
        match &self.component_type {
            ComponentType::Text(text) => {
                let text = normalize_text(&text);

                let formatted = apply_style(Colorize::clear(text.as_str()), self.style.clone());

                let formatted = match formatted {
                    Ok(f) => f,
                    Err(e) => return io::Result::Err(io::Error::new(io::ErrorKind::InvalidData, e))
                };

                let formatted = formatted.color(self.color.clone());
                let formatted = format!("{}", formatted);

                (*stdout).write(formatted.as_bytes())
            },
            ComponentType::Box(cmps) => {
                for cmp in cmps.to_vec().iter() {
                    //(*stdout).write(format!("CMP: {:?}\n", &cmp).as_bytes()).unwrap();

                    match Drawable::draw_(cmp, Box::new(*stdout)) {
                        io::Result::Err(e) => panic!("Draw error: {}", e),
                        _ => ()
                    };
                }

                return io::Result::Ok(0);
            },
            #[allow(unreachable_patterns)]
            _ => io::Result::Err(io::Error::new(io::ErrorKind::NotFound, "The draw method is not implemented in this type"))
        }
    }
}

