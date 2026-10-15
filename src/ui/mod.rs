use crate::trait_functions;

impl trait_functions::UIShownWindow for trait_functions::utils::UIElementsShow<bool, String> {
    fn show_ui_screen(&mut self) -> Self {
        let t: trait_functions::utils::UIElementsShow<bool, String> =
            trait_functions::utils::UIElementsShow {
                created: String::from("-"),
                is_shown: true,
                length: 0.0,
                xpos: self.xpos as f32,
            };

        t
    }
}
