use termion::event::Key;

use crate::widget::{Widget, WidgetCore};
use crate::fail::{HResult, ErrorLog};

pub trait Tabbable {
    fn new_tab(&mut self) -> HResult<()>;
    fn close_tab(&mut self) -> HResult<()>;
    fn next_tab(&mut self) -> HResult<()>;
    fn on_next_tab(&mut self) -> HResult<()> {
        Ok(())
    }
    fn get_tab_names(&self) -> Vec<Option<String>>;
    fn active_tab(&self) -> &dyn Widget;
    fn active_tab_mut(&mut self) -> &mut dyn Widget;
    fn on_key_sub(&mut self, key: Key) -> HResult<()>;
    fn on_key(&mut self, key: Key) -> HResult<()> {
        match key {
            Key::Ctrl('t') => self.new_tab(),
            Key::Ctrl('w') => self.close_tab(),
            Key::Char('\t') => self.next_tab(),
            _ => self.on_key_sub(key)
        }
    }
}


#[derive(PartialEq)]
pub struct TabView<T> where T: Widget, TabView<T>: Tabbable {
    pub widgets: Vec<T>,
    pub active: usize,
    core: WidgetCore
}

impl<T> TabView<T> where T: Widget, TabView<T>: Tabbable {
    pub fn new(core: &WidgetCore) -> TabView<T> {
        TabView {
            widgets: vec![],
            active: 0,
            core: core.clone()
        }
    }

    pub fn push_widget(&mut self, widget: T) -> HResult<()> {
        self.widgets.push(widget);
        self.refresh()
    }

    pub fn pop_widget(&mut self) -> HResult<T> {
        let widget = self.widgets.pop()?;
        self.refresh()?;
        Ok(widget)
    }

    pub fn active_tab_(&self) -> &T {
        &self.widgets[self.active]
    }

    pub fn active_tab_mut_(&mut self) -> &mut T {
        &mut self.widgets[self.active]
    }

    pub fn close_tab_(&mut self) -> HResult<()> {
        self.pop_widget()?;
        self.active -= 1;
        Ok(())
    }

    pub fn next_tab_(&mut self) {
        if self.active + 1 == self.widgets.len() {
            self.active = 0;
        } else {
            self.active += 1
        }
        self.on_next_tab().log();
    }
}

impl<T> Widget for TabView<T> where T: Widget, TabView<T>: Tabbable {
    fn get_core(&self) -> HResult<&WidgetCore> {
        Ok(&self.core)
    }
    fn get_core_mut(&mut self) -> HResult<&mut WidgetCore> {
        Ok(&mut self.core)
    }
    fn render_header(&self) -> HResult<String> {
        let xsize = self.get_coordinates()?.xsize();
        let header = self.active_tab_().render_header()?;
        let tab_names = self.get_tab_names();
        let mut nums_length = 0;
        let tabnums = (0..self.widgets.len()).map(|num| {
            nums_length += format!("{}:{} ",
                                   num,
                                   tab_names[num].as_ref().unwrap()).len();
            if num == self.active {
                format!(" {}{}:{}{}{}",
                        crate::term::invert(),
                        num,
                        tab_names[num].as_ref().unwrap(),
                        crate::term::reset(),
                        crate::term::header_color())
            } else {
                format!(" {}:{}", num, tab_names[num].as_ref().unwrap())
            }
        }).collect::<String>();

        let nums_pos = xsize - nums_length as u16;

        Ok(format!("{}{}{}{}",
                header,
                crate::term::header_color(),
                crate::term::goto_xy(nums_pos, 1),
                tabnums))
    }

    fn render_footer(&self) -> HResult<String>
    {
        self.active_tab_().render_footer()
    }

    fn refresh(&mut self) -> HResult<()> {
        self.active_tab_mut().refresh()
    }

    fn get_drawlist(&self) -> HResult<String> {
        self.active_tab_().get_drawlist()
    }

    fn on_key(&mut self, key: Key) -> HResult<()> {
        Tabbable::on_key(self, key)?;
        self.refresh()
    }
}