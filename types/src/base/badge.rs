pub struct Badge {
    pub text: String,
    pub badge_type: BadgeColor,
}

pub enum BadgeColor {
    Default,
    Success,
    Info,
    Warning,
    Danger,
}
