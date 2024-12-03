use charming::{
    component::{Legend, Title},
    element::{Orient, Tooltip, Trigger},
    series::Pie,
    Chart, HtmlRenderer,
};

#[derive(Debug, Clone, Copy)]
pub struct ChartSize {
    pub width: u64,
    pub height: u64,
}

impl ChartSize {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }
}

pub fn create(title: impl Into<String>, langs: Vec<(f64, String)>, size: &ChartSize) -> String {
    let title: String = title.into();
    let chart = Chart::new()
        .title(Title::new().text(title.clone()).left("center"))
        .legend(Legend::new().orient(Orient::Vertical).left("left"))
        .tooltip(Tooltip::new().trigger(Trigger::Item))
        .series(Pie::new().name(title.clone()).radius(100.0).data(langs));

    let renderer = HtmlRenderer::new("main_chart", size.width, size.height)
        .theme(charming::theme::Theme::Walden);
    renderer.render(&chart).unwrap()
}
