use winit::error::{EventLoopError, OsError};
use crate::rendering::RenderingQueueError;

#[derive(Debug)]
pub enum ApplicationError{
    EventLoopError(EventLoopError),
    WindowError(OsError),
    RenderingQueueError(RenderingQueueError)
}

impl From<EventLoopError> for ApplicationError {
    fn from(error: EventLoopError) -> Self {
        ApplicationError::EventLoopError(error)
    }
}

impl From<RenderingQueueError> for ApplicationError {
    fn from(error: RenderingQueueError) -> Self {
        ApplicationError::RenderingQueueError(error)
    }
}

impl From<OsError> for ApplicationError {
    fn from(error: OsError) -> Self {
        ApplicationError::WindowError(error)
    }
}