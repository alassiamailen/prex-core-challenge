/// Type of errors
#[derive(Debug, PartialEq)]
pub enum CommonError {
    NotFound,
    FolderCreationFailed,
    FolderReadFailed,
    LockReadFailed,
    LockWriteFailed,
    FileCreationFailed,
    FileWriteFailed,
    Forbiden,
}
