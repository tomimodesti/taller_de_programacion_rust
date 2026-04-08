//! Modulo de errores: maneja todo lo relacionado a posibles errores que pueden surgir
//! al ejecutar minikv, desde errores del cliente a errores del servidor a
//! errores de comunicacion entre ambos

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
    Timeout,
    ClientSocketBinding,
    InvalidArgs,
    InvalidPuerto,
    EOF,
    EMPTY,
    InvalidDataFile,
    InvalidLogFile,
    //error default, errores que son especificos
    Error(String),
}

impl KvErrores {
    pub fn es_recuperable(&self) -> bool {
        matches!(
            self,
            Self::NotFound | Self::ExtraArgument | Self::MissingArgument | Self::UnknownCommand
        )
    }
    pub fn to_str(&self) -> String {
        match self {
            Self::NotFound => "Not Found".to_string(),
            Self::ExtraArgument => "EXTRA ARGUMENT".to_string(),
            Self::MissingArgument => "MISSING ARGUMENT".to_string(),
            Self::UnknownCommand => "UNKNOWN COMMAND".to_string(),
            Self::IoError(_) => "IoError".to_string(),
            Self::ConnectionClosed => "CONNECTION CLOSED".to_string(),
            Self::Timeout => "Timeout".to_string(),
            Self::ClientSocketBinding => "CLIENT SOCKET BINDING".to_string(),
            Self::InvalidArgs => "INVALID ARGS".to_string(),
            Self::InvalidPuerto => "Invalid Puerto".to_string(),
            Self::EOF => "EOF".to_string(),
            Self::EMPTY => "EMPTY".to_string(),
            Self::InvalidDataFile => "INVALID DATA FILE".to_string(),
            Self::InvalidLogFile => "INVALID LOG FILE".to_string(),
            Self::Error(mensaje) => mensaje.clone(),
        }
    }
}
