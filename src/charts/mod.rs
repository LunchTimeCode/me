use charming::{
    component::Legend,
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, HtmlRenderer,
};

pub fn get(langs: Vec<(f64, String)>) -> String {
    let chart = Chart::new().legend(Legend::new().top("bottom")).series(
        Pie::new()
            .name("Languages")
            .rose_type(PieRoseType::Area)
            .radius(vec!["50", "250"])
            .center(vec!["50%", "50%"])
            .item_style(ItemStyle::new().border_radius(8))
            .data(langs),
    );

    let renderer =
        HtmlRenderer::new("my charts", 1000, 800).theme(charming::theme::Theme::Halloween);
    renderer.render(&chart).unwrap()
}
