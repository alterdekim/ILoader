use cacao::{appkit::toolbar::Toolbar, button::Button, layout::Layout, view::{View, ViewDelegate}};

pub struct ScreenView {
    //pub toolbar: Toolbar
    pub btn: Button
}

impl ScreenView {
    pub fn new() -> Self {
        //let toolbar = Toolbar::new("toolbar", delegate);
        let btn = Button::new("test");

        Self { btn }
    }
}

impl ViewDelegate for ScreenView {
    const NAME: &'static str = "ScreenView";

    fn did_load(&mut self, view: View) {
        view.add_subview(&self.btn);
    }
}