#[derive(Debug)]
pub enum CommonError {
    NotFound,
    InternalServerError,
    FolderCreationFailed,
    FolderReadFailed,
    LockReadFailed,
    LockWriteFailed,
    FileCreationFailed,
    FileWriteFailed,  
    Forbiden,
}
