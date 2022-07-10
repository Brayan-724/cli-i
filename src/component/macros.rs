#[macro_export]
macro_rules! create_text {
    (@itype $text:ident) => ($crate::component::ComponentType::Text($text));
    (@itype $text:expr) => ($crate::component::ComponentType::Text(String::from($text)));
    (@type $text:tt) => ($crate::component::create_text!(@itype $text));

    (@ivalue $val:ident) => ($val);
    (@ivalue $val:expr) => ($val);
    (@value $val:tt) => ($crate::component::create_text!(@ival $val));

    ($text:expr, $color:expr, $style:expr) => ({
        let component_type = $crate::component::create_text!(@type $text);
        Component {
            component_type,
            is_static: true,
            color: $color,
            style: $style
        }
    });

    ($text:expr, $color:expr) => ( $crate::component::create_text!($text, $color, Styles::Clear) );
    ($text:expr) => ( $crate::component::create_text!($text, Color::White) );
}

#[macro_export]
macro_rules! create_box {
    (@interc text $($val:expr),+) => ($crate::component::create_text!($($val),+));

    (@v $($c:expr)? ; $vec:ident) => {{
        let vec: Vec<$crate::component::Component> = $vec;
        let cmps = vec.clone().into_boxed_slice();
        let is_interactive = vec.iter().any(|cmp| !cmp.is_static);
        let is_static = !is_interactive;
        let component_type = $crate::component::ComponentType::Box(cmps);

        $crate::component::Component {
            component_type,
            is_static,
            color: $crate::component::handle_optional!({ $($c)? } else { Color::White }),
            style: Styles::Clear 
        }
    }};

    (@b text $($val: expr),+; $($rest:tt)*) => {{
        let cmps = $crate::component::create_box!(@interc text $($val),*);
        $crate::component::create_box!(@c cmps $($rest)*)
    }};

    (@b paragraph $color:expr, { $($txt:tt)* }; $($rest:tt)*) => {{
        let cmps = $crate::component::create_box!(! $color ; $($txt)* text "\n";);
        $crate::component::create_box!(@c cmps $($rest)*)
    }};
    
    (@b) => (Vec::<$crate::component::Component>::new());

    (@c $cmps:ident $($rest:tt)*) => {{
        let vec_cmps: Vec<$crate::component::Component> = vec![$cmps];

        let v: Vec<$crate::component::Component> = $crate::component::create_box!(@b $($rest)*);
        let vec_ = $crate::common::vec_concat!(vec_cmps, v);

        vec_.collect()
    }};

    (! $($c:expr)? ; $($v:tt)*) => ({
        let vec_ = $crate::component::create_box!(@b $($v)*);
        $crate::component::create_box!(@v $($c)? ; vec_)
    });
}

#[macro_export]
macro_rules! handle_optional {
    ({ $($T:tt)+ } else { $($F:tt)* }) => ($($T)*);
    ({ } else { $($F:tt)+ }) => ($($F)*);
}

pub use create_text;
pub use create_box;
pub use handle_optional;
