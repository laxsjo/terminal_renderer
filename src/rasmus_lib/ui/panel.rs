use crate::math::*;
use std::any;

// use super::events::*;
use super::*;

pub trait Panel: std::fmt::Debug {
    #[allow(unused)]
    fn tick(&mut self, key_events: &InputStream) {}

    fn render(&mut self) -> Render;

    fn as_any(&self) -> &dyn any::Any;
    fn as_any_mut(&mut self) -> &mut dyn any::Any;
}

#[derive(Debug)]
pub struct TextPanel {
    contents: RenderBuffer,
}

impl TextPanel {
    pub fn new(contents: String) -> Self {
        Self {
            contents: RenderBuffer::from_string(contents),
        }
    }

    pub fn create_panel(pos: UVec2, contents: String) -> PanelEntity {
        PanelEntity::new(pos, Self::new(contents))
    }

    pub fn set_contents(&mut self, contents: String) {
        self.contents = RenderBuffer::from_string(contents);
    }
}

impl Panel for TextPanel {
    fn render(&mut self) -> Render {
        Render::new(self.contents.clone())
    }

    fn as_any(&self) -> &dyn any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn any::Any {
        self
    }
}

#[derive(Debug)]
pub struct PanelEntity {
    pub pos: UVec2,
    panel: Box<dyn Panel>,
}
impl PanelEntity {
    pub fn new<T>(pos: UVec2, panel: T) -> Self
    where
        T: Panel + 'static,
    {
        Self {
            pos,
            panel: Box::new(panel),
        }
    }

    pub fn tick(&mut self, key_events: &InputStream) {
        self.panel.tick(key_events);
    }

    pub fn render(&mut self) -> Render {
        self.panel.render()
    }

    pub fn as_panel<T: Panel + 'static>(&self) -> Option<&T> {
        self.panel.as_any().downcast_ref()
    }

    pub fn as_panel_mut<T: Panel + 'static>(&mut self) -> Option<&mut T> {
        self.panel.as_any_mut().downcast_mut()
    }
}

// impl events::EventReceiver for Panel {
//     fn receive_event<T>(&mut self, event: events::Event<T, Self>) {}
// }

// pub struct PanelKeyEventHandler
