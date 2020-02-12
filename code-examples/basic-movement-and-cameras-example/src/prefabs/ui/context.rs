use std::borrow::Borrow;

use amethyst::assets::AssetPrefab;
use amethyst::ui::Anchor;
use amethyst::ui::FontAsset;
use amethyst::ui::NoCustomUi;
use amethyst::ui::TtfFormat;
use amethyst::ui::UiTextData;
use amethyst::ui::UiTransformData;
use amethyst::ui::UiWidget;
use ron::ser::{to_string_pretty, PrettyConfig};

type UiWidgetNoCustom = UiWidget<NoCustomUi, u32, ()>;

pub enum ContextLabelId {
    CameraFirstPerson,
    CameraThirdPerson,
    CameraTopDown,
    CameraIsometric,
    CameraSideScroller,
    MovementTranslational,
    MovementFreeFloat,
}

impl<'a> ContextLabelId {
    pub fn id(self) -> &'a str {
        return match self {
            ContextLabelId::CameraFirstPerson => "camera_first_person",
            ContextLabelId::CameraThirdPerson => "camera_third_person",
            ContextLabelId::CameraTopDown => "camera_top_down",
            ContextLabelId::CameraIsometric => "camera_isometric",
            ContextLabelId::CameraSideScroller => "camera_side_scroller",
            ContextLabelId::MovementTranslational => "movement_translational",
            ContextLabelId::MovementFreeFloat => "movement_free_float",
        };
    }
}

pub fn build_context_ui_prefab() -> String {
    let font: AssetPrefab<FontAsset> =
        AssetPrefab::File("font/square.ttf".to_owned(), Box::new(TtfFormat));

    let label_offset: f32 = -25.0;

    let labels = vec![
        ("Camera angles", None),
        (
            "First-person angle: [1]",
            Some(ContextLabelId::CameraFirstPerson.id()),
        ),
        (
            "Third-person angle: [2]",
            Some(ContextLabelId::CameraThirdPerson.id()),
        ),
        (
            "Top-down angle: [3]",
            Some(ContextLabelId::CameraTopDown.id()),
        ),
        (
            "Isometric angle: [4]",
            Some(ContextLabelId::CameraIsometric.id()),
        ),
        (
            "Side-scroller angle: [5]",
            Some(ContextLabelId::CameraSideScroller.id()),
        ),
        ("", None),
        ("Movement types", None),
        (
            "Translational movement: [q]",
            Some(ContextLabelId::MovementTranslational.id()),
        ),
        (
            "'Free-float' movement: [e]",
            Some(ContextLabelId::MovementFreeFloat.id()),
        ),
    ];

    let ui_widgets: Vec<UiWidgetNoCustom> = labels
        .iter()
        .enumerate()
        .map(|(index, (text, id))| UiWidget::Label {
            transform: single_line_top_left_ui_transform(label_offset * index as f32, *id),
            text: regular_text(text, font.clone()),
        })
        .collect();

    let container_transform = {
        let mut transform = UiTransformData::default()
            .with_position(20., -100., 0.)
            .with_size(300., 100.)
            .with_id("context_container")
            .with_anchor(Anchor::TopLeft);
        transform.pivot = Anchor::TopLeft;
        transform
    };
    let container: UiWidgetNoCustom = UiWidget::Container {
        transform: container_transform,
        background: None,
        children: ui_widgets,
    };

    let pretty = PrettyConfig {
        depth_limit: 2,
        new_line: "".to_string(),
        indentor: "".to_string(),
        separate_tuple_members: true,
        enumerate_arrays: true,
    };

    return to_string_pretty(&container, pretty).expect("Serialization failed");
}

fn regular_text(text: &str, font: AssetPrefab<FontAsset>) -> UiTextData {
    UiTextData {
        text: text.to_owned(),
        editable: None,
        font_size: 20.0,
        password: false,
        color: [0., 0., 0., 1.],
        align: None,
        line_mode: None,
        font: Some(font),
    }
}

fn single_line_top_left_ui_transform(y_offset: f32, id: Option<&str>) -> UiTransformData<()> {
    let mut transform = UiTransformData::default()
        .with_position(0., y_offset, 0.)
        .with_size(300., 25.)
        .with_anchor(Anchor::TopLeft);
    transform.pivot = Anchor::TopLeft;
    if let Some(id) = id {
        transform.id = id.to_owned();
    }
    transform
}
