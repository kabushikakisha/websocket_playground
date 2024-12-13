use std::fmt::{Formatter, Display};


// represents potential errors that can occur during WS creation process
pub enum ConnectionError {
    // indicates an error occurred while creating the WS server
    CreateServerError(String), // stores a description of the server creation error
}

impl Display for ConnectionError {

    // formats the ConnectionError for display as a human-readable string
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self { // pattern match on the CreateServerError
            ConnectionError::CreateServerError(ref desc) => {
                write!(f, "Error while creating server. {}", desc)
            }
        }
    }

}