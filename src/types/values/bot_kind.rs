use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum BotKind {
    Single,
    Multi,
}
