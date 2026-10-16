pub trait UIShownWindow {
    fn show_ui_screen(&mut self) -> Self;
}

pub mod utils {
    pub struct UIElementsShow<CreatedUI> {
        // pub is_shown: ElmementsShown,
        pub created: CreatedUI,
        pub xpos: f32,
    }
}
