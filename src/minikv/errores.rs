

#[derive(Debug)]
pub enum KvErrores {
    //RECUPERABLES:
    NotFound,
    ExtraArgument,
    MissingArgument,
    UnknownCommand,

    //Irrecuperables:
    IoError(String),
    ConnectionClosed,
}

impl KvErrores {
    pub fn es_recuperable (&self) -> bool {
        match self {
        Self::NotFound | Self::ExtraArgument | Self::MissingArgument | Self::UnknownCommand => true,
        _ => false,
        }
    }
}