pub mod ui;

pub enum PrefabFilename {
    UiFpsCounter,
    UiContext,
}

impl<'a> PrefabFilename {
    pub fn filename(self) -> &'a str {
        return prefab_filename(self);
    }
}

pub fn prefab_filename<'a>(prefab_filename: PrefabFilename) -> &'a str {
    return match prefab_filename {
        PrefabFilename::UiFpsCounter => "fps.ron",
        PrefabFilename::UiContext => "context.ron",
    };
}
