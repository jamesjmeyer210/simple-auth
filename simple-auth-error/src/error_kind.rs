/// The kind of error
#[derive(Debug)]
pub enum ErrorKind {
    /// There error is not defined
    Undefined,
    /// The operation was cancelled
    Cancelled,
    /// The argument was invalid (400)
    InvalidArgument,
    /// Authentication is required to perform this action (401)
    Unauthorized,
    /// The permissions to perform the operation or access the resource are not sufficient (403)
    Forbidden,
    /// The resource was not found (404)
    NotFound,
    DeadlineExceeded,
    AlreadyExists,
    ResourceExhausted,
    FailedPrecondition,
    Aborted,
    OutOfRange,
    /// An error internal to the application occurred (500)
    Internal,
    /// The operation cannot be performed because it does not have an implementation (501)
    Unimplemented,
    /// The resource is not available for the required action (503)
    Unavailable,
    DataLoss,
    Unknown,
}