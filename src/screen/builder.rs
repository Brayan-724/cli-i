use super::Screen;
use crate::component::Component;

pub struct ScreenBuilder {
    is_static: bool,
    auto_clear: bool,
    components: Vec<&'static dyn Component>,
}

impl ScreenBuilder {
    pub fn new() -> ScreenBuilder {
        ScreenBuilder {
            is_static: true,
            auto_clear: true,
            components: Vec::new()
        }
    }

    pub fn set_auto_clear(&mut self, auto_clear_: bool) -> &mut Self {
        self.auto_clear = auto_clear_;
        self
    }

    pub fn add_component<C: Component>(&mut self, component: &'static C) -> &mut Self {
        let mut components = self.components.clone();
        let next_is_static = component.get_is_static();
        components.push(component);

        if self.is_static {
            self.is_static = next_is_static;
        }

        self.components = components;

        self
    }

    pub fn build(&self) -> Screen {
        let components = &self.components;
        let components = components.clone().into_boxed_slice();

        Screen {
            is_static: self.is_static,
            auto_clear: self.auto_clear,
            components
        }
    }
}
