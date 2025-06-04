use crate::base::LanguageKey;

pub enum SettingType {
    String(String),
    Boolean(bool),
    Number(f64),
    LanguageKey(LanguageKey),
}

pub struct SourceSetting {
    pub key: String,
    pub name: String,
    pub description: String,
    pub default_type: SettingType,
    pub required: bool,
}
