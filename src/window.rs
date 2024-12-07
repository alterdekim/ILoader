pub mod main_window {
    use cacao::{appkit::window::{TitleVisibility, Window, WindowDelegate}, view::SplitViewController};

    use crate::view::{content_view::ScreenView, details_view::Details, sidebar::MainSidebar};

    #[derive(Default)]
    pub struct MainWindow {
        split_view_controller: Option<SplitViewController<MainSidebar, ScreenView, Details>>
    }

    impl WindowDelegate for MainWindow {
        const NAME: &'static str = "MainWindow";

        fn did_load(&mut self, window: Window) {
            window.set_title("ILoader");
            window.set_title_visibility(TitleVisibility::Hidden);
            window.set_titlebar_appears_transparent(true);
            window.set_movable_by_background(true);
            window.set_autosave_name("CacaoILoader");
            window.set_minimum_content_size(980., 700.);

            let split_view_controller = SplitViewController::new(MainSidebar::default(),
            ScreenView::default(),
            Some(Details::default())); // Some(DetailView::default())

            window.set_content_view_controller(&split_view_controller);

            self.split_view_controller = Some(split_view_controller);
        }
    }
}