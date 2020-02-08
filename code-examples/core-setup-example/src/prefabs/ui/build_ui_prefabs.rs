use amethyst::ui::UiWidget;
use amethyst::ui::NoCustomUi;
use amethyst::ui::UiTransformData;
use amethyst::ui::UiTextData;
use amethyst::ui::FontAsset;
use amethyst::ui::TtfFormat;
use amethyst::ui::Anchor;
use amethyst::assets::AssetPrefab;
use ron::ser::{to_string_pretty, PrettyConfig};

pub fn build_ui_prefabs() -> String {
    let font: Option<AssetPrefab<FontAsset>> = Some(AssetPrefab::File("font/square.ttf".to_owned(), Box::new(TtfFormat)));

    let fps_text = UiTextData {
        text: "N/A".to_owned(),
        editable: None,
        font_size: 20.0,
        password: false,
        color: [0., 0., 0., 1.],
        align: None,
        line_mode: None,
        font,
    };

    let fps_label: UiWidget<NoCustomUi, u32, ()> = UiWidget::Label {
        transform: UiTransformData::default()
            .with_position(60., -30., 0.)
            .with_size(200., 25.)
            .with_id("fps")
            .with_anchor(Anchor::TopLeft),
        text: fps_text,
    };

    let pretty = PrettyConfig {
        depth_limit: 2,
        new_line: "".to_string(),
        indentor: "".to_string(),
        separate_tuple_members: true,
        enumerate_arrays: true,
    };

    let s = to_string_pretty(&fps_label, pretty).expect("Serialization failed");

    return s;
}