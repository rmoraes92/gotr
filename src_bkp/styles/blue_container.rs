use iced::{gradient, widget::container, Radians};

pub struct BlueContainerStyleSheet;

impl BlueContainerStyleSheet {
    pub fn new() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(Self))
    }
}

impl container::StyleSheet for BlueContainerStyleSheet {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let palette = style.palette();
        let bg = palette.primary;
        let fg = palette.primary;
        let gradient = gradient::Linear::new(Radians(90.0))
            .add_stop(0.0, bg)
            .add_stop(1.0, fg)
            .into();
        container::Appearance {
            text_color: Some(palette.text),
            background: Some(iced::Background::Gradient(gradient)),
            ..Default::default()
        }
    }
}
