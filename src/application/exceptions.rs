use winit::error::{EventLoopError, OsError};
use crate::rendering::RenderingError;

#[derive(Debug)]
pub enum ApplicationError{
    EventLoopError(EventLoopError),
    WindowError(OsError),
    RenderingError(RenderingError)
}

impl From<EventLoopError> for ApplicationError {
    fn from(error: EventLoopError) -> Self {
        ApplicationError::EventLoopError(error)
    }
}

impl From<RenderingError> for ApplicationError {
    fn from(error: RenderingError) -> Self {
        ApplicationError::RenderingError(error)
    }
}

impl From<OsError> for ApplicationError {
    fn from(error: OsError) -> Self {
        ApplicationError::WindowError(error)
    }
}