//generic error


#[derive(Debug)]
pub struct GenericError{
    pub message: String,
}

impl Error for GenericError {}

impl Display
