use crate::trait_functions;

impl trait_functions::UIShownWindow for trait_functions::utils::UIElementsShow<String> {
    fn show_ui_screen(&mut self) -> Self {
        let t: trait_functions::utils::UIElementsShow<String> =
            trait_functions::utils::UIElementsShow {
                created: String::from("-"),
                xpos: self.xpos as f32,
            };

        t
    }
}
