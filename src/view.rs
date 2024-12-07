
pub mod sidebar {
    use cacao::{appkit::FocusRingType, button::{BezelStyle, Button}, geometry::Rect, layout::Layout, text::Font, view::{View, ViewDelegate}};

    #[derive(Default)]
    pub struct MainSidebar {

    }

    impl ViewDelegate for MainSidebar {
        const NAME: &'static str = "MainSidebar";

        fn did_load(&mut self, view: View) {
            let mut btn = Button::new("testtesttest");
            btn.set_bezel_style(BezelStyle::TexturedRounded);
            btn.set_bordered(false);
            btn.set_font(Font::system(14.));
            btn.set_action(|| {
                println!("HEY");
            });
            view.add_subview(&btn);
        }
    }
}

pub mod content_view {
    use cacao::{button::Button, layout::Layout, view::{View, ViewDelegate}};

    pub struct ScreenView {
        pub btn: Button
    }

    impl Default for ScreenView {
        fn default() -> Self {
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
}

pub mod details_view {
    use cacao::{button::Button, layout::Layout, view::{View, ViewDelegate}};

    #[derive(Default)]
    pub struct Details {

    }

    impl ViewDelegate for Details {
        const NAME: &'static str = "Details";
    }
}