use super::*;
use crate::ansi_term::*;
use crate::math::UVec2;
// use crate::events;
use crate::flags::TerminalFlags;
use crate::utils::{Ref, RefVec};
use crate::CleanUp;

const MAX_FPS: f32 = 20.;

pub type PanelRef = Ref<PanelEntity>;

pub trait TickerFn: FnMut(&mut RefVec<PanelEntity>, &InputStream) {}

impl<T> TickerFn for T where T: FnMut(&mut RefVec<PanelEntity>, &InputStream) {}

pub struct Gui<'a> {
    panels: RefVec<PanelEntity>,
    inputs: InputManager<'a>,
    external_tickers: Vec<Box<dyn TickerFn>>,
    _clean_up: CleanUp,
}

impl<'a> Gui<'a> {
    pub fn new() -> Self {
        TerminalFlags::set_alternative_buffer(true);
        // screen::activate_alternative_buffer();
        screen::clear();
        flush_commands();

        Self {
            panels: RefVec::new(),
            inputs: InputManager::new(),
            external_tickers: Vec::new(),
            _clean_up: CleanUp,
        }
    }

    pub fn add_panel(&mut self, pos: UVec2, panel: impl Panel + 'static) -> PanelRef {
        self.panels.push(PanelEntity::new(pos, panel))
    }

    pub fn remove_panel(&mut self, panel: &PanelRef) -> Option<PanelEntity> {
        self.panels.remove(panel)
    }

    pub fn get_panel<T: Panel + 'static>(&self, panel: &PanelRef) -> Option<&T> {
        self.panels.get(panel)?.as_panel()
    }
    pub fn get_panel_mut<T: Panel + 'static>(&mut self, panel: &PanelRef) -> Option<&mut T> {
        self.panels.get_mut(panel)?.as_panel_mut()
    }

    pub fn tick(&mut self) {
        let before = std::time::SystemTime::now();

        let key_events = self.inputs.tick();

        for ticker in &mut self.external_tickers {
            ticker(&mut self.panels, &key_events);
        }

        let mut buffer = RenderBuffer::new();
        // buffer.push(&screen::get_clear());
        buffer.push(&screen::get_fill_with_char(' '));
        buffer.push(&cursor::get_hide());

        buffer.push(&cursor::get_move_home());
        // buffer.push(&format!("panels: {:?}", &self.panels));

        for panel in self.panels.iter_mut() {
            panel.tick(&key_events);
            let render = panel.render();

            let pos = panel.pos;

            buffer.push(&cursor::get_move_to_position(pos.x as u16, pos.y as u16));

            render.print(&mut buffer, pos.x as u16);
        }

        buffer.push(&cursor::get_move_home());

        flush_commands();
        // Source: https://gitlab.com/gnachman/iterm2/-/wikis/synchronized-updates-spec
        // They didn't work :(
        // print!("\x1b[?2026h");
        // print!("\x1bP=1s\x1b\\");
        print!("{}", buffer.content);
        // print!("\x1b[?2026l");
        // print!("\x1bP=2s\x1b\\");
        flush_commands();

        const MIN_DELTA_SECS: f32 = 1. / MAX_FPS;

        let duration = before.elapsed().expect("time went backwards");
        let delta_secs = duration.as_secs_f32();

        let sleep_secs = (MIN_DELTA_SECS - delta_secs).max(0.);

        std::thread::sleep(std::time::Duration::from_secs_f32(sleep_secs));
    }

    pub fn add_ticker(
        &mut self,
        tick: impl FnMut(&mut RefVec<PanelEntity>, &InputStream) + 'static,
    ) {
        self.external_tickers.push(Box::new(tick));
    }
}

impl<'a> Default for Gui<'a> {
    fn default() -> Self {
        Self::new()
    }
}

// impl events::EventManager<>
