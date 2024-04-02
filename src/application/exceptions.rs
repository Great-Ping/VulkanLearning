use winit::error::{EventLoopError, OsError};

use crate::rendering::{
    RenderingQueue
};

#[derive(Debug)]
pub enum ApplicationError{
    EventLoopError(EventLoopError),
    WindowError(OsError)
}

impl From<EventLoopError> for ApplicationError {
    fn from(error: EventLoopError) -> Self {
        ApplicationError::EventLoopError(error)
    }
}

impl From<OsError> for ApplicationError {
    fn from(error: OsError) -> Self {
        ApplicationError::WindowError(error)
    }
}