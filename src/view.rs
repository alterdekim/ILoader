
pub mod sidebar {
    use cacao::view::{View, ViewDelegate};

    #[derive(Default)]
    pub struct MainSidebar {

    }

    impl ViewDelegate for MainSidebar {
        const NAME: &'static str = "MainSidebar";

        fn did_load(&mut self, _view: View) {
            
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