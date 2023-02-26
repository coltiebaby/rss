pub mod skins;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ChampSelect {
    #[default]
    Unknown,
    CarouselSkins(skins::CarouselSkin),
    SelectSkin(skins::Selected),
}
