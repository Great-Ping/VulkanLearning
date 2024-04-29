use winit::error::{EventLoopError, OsError};
use crate::pipeline_builder::RenderingError;

#[derive(Debug)]
pub enum ApplicationError{
    EventLoopError(EventLoopError),
    WindowError(OsError),
    RenderingQueueError(RenderingError)
}

impl From<EventLoopError> for ApplicationError {
    fn from(error: EventLoopError) -> Self {
        ApplicationError::EventLoopError(error)
    }
}

impl From<RenderingError> for ApplicationError {
    fn from(error: RenderingError) -> Self {
        ApplicationError::RenderingQueueError(error)
    }
}

impl From<OsError> for ApplicationError {
    fn from(error: OsError) -> Self {
        ApplicationError::WindowError(error)
    }
}