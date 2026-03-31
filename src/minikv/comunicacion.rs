pub enum ResultadoComunicacion {
    Continuar(String), //respuesta o error recuperable
    Cerrar(String),    //error no recuperable o EOF
}
