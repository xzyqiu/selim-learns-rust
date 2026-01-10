#[derive(Debug, Clone)]
pub enum Oath {
    Bladesharper,
    Oathless,
    Jetstriker,
    Saltchemist,
}

impl Oath {
    pub fn info(&self) -> &str {
        match self {
            Oath::Bladesharper => "Increases weapon damage.",
            Oath::Jetstriker => "Increases attack speed.",
            Oath::Saltchemist => "Enhances potion effects.",
            Oath::Oathless => "Does nothing.",
        }
    }
}
