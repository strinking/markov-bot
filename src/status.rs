use serenity::model::OnlineStatus;

pub enum Status {}

impl Status {
    pub fn from_str(name: &str) -> Result<OnlineStatus, ()> {
        match name.to_lowercase().as_str() {
            "online" | "reset" => Ok(OnlineStatus::Online),
            "invisible" | "invis" => Ok(OnlineStatus::Invisible),
            "dnd" => Ok(OnlineStatus::DoNotDisturb),
            "idle" => Ok(OnlineStatus::Idle),
            _ => Err(())
        }
    }
}