use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub text: String,
    pub badge_type: BadgeColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BadgeColor {
    Default,
    Success,
    Info,
    Warning,
    Danger,
}

impl BadgeColor {
    pub fn to_str(&self) -> &'static str {
        match self {
            BadgeColor::Default => "default",
            BadgeColor::Success => "success",
            BadgeColor::Info => "info",
            BadgeColor::Warning => "warning",
            BadgeColor::Danger => "danger",
        }
    }
}
