pub trait UIShownWindow {
    fn show_ui_screen(&mut self) -> Self;
}

pub mod utils {
    pub struct UIElementsShow<ElmementsShown, CreatedUI> {
        pub is_shown: ElmementsShown,
        pub created: CreatedUI,
        pub length: f32,
        pub xpos: f32,
    }
}
