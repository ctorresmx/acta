//! # Persistence
//!
//! `persistence` contains the functions to manipulate the todo data.
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::model::Todo;
use dirs::home_dir;

/// The default filename for the todo storage in the user's home directory
const FILE_NAME: &str = ".acta";

/// Custom error type for persistence operations
///
/// This enum encapsulates all possible errors that can occur during
/// file persistence operations, including I/O errors, JSON parsing errors,
/// and missing home directory errors.
#[derive(Debug)]
pub enum ActaError {
    /// Wraps standard I/O errors from file operations
    Io(io::Error),
    /// Wraps JSON serialization/deserialization errors
    Parse(serde_json::Error),
    /// Indicates that the user's home directory could not be determined
    HomeDir,
}

impl fmt::Display for ActaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActaError::Io(e) => write!(f, "IO error: {}", e),
            ActaError::Parse(e) => write!(f, "Parse error: {}", e),
            ActaError::HomeDir => write!(f, "Could not find home directory"),
        }
    }
}

impl From<io::Error> for ActaError {
    fn from(err: io::Error) -> ActaError {
        ActaError::Io(err)
    }
}

impl From<serde_json::Error> for ActaError {
    fn from(err: serde_json::Error) -> ActaError {
        ActaError::Parse(err)
    }
}

impl std::error::Error for ActaError {}

/// Initializes the storage file in the home directory
///
/// This function locates the user's home directory and creates the todo
/// storage file if it doesn't already exist. If the file exists, it leaves
/// it untouched.
///
/// # Returns
///
/// Returns the full path to the storage file on success.
///
/// # Errors
///
/// - `ActaError::HomeDir` if the home directory cannot be determined
/// - `ActaError::Io` if the file cannot be created
///
/// # Examples
///
/// ```no_run
/// use acta::persistence::init;
///
/// let path = init().expect("Failed to initialize storage");
/// println!("Storage initialized at: {:?}", path);
/// ```
pub fn init() -> Result<PathBuf, ActaError> {
    let mut file_path = home_dir().ok_or(ActaError::HomeDir)?;
    file_path.push(FILE_NAME);

    if !file_path.exists() {
        fs::write(&file_path, "[]")?;
    }

    Ok(file_path)
}

/// Reads the todo items from the default storage file
///
/// This function reads and deserializes the todo list from the storage file.
/// If the file doesn't exist, it will be created with an empty list via `init()`.
///
/// # Returns
///
/// Returns a vector of `Todo` items on success.
///
/// # Errors
///
/// - `ActaError::HomeDir` if the home directory cannot be determined
/// - `ActaError::Io` if the file cannot be read
/// - `ActaError::Parse` if the JSON content is malformed
///
/// # Examples
///
/// ```no_run
/// use acta::persistence::read;
///
/// let todos = read().expect("Failed to read todos");
/// println!("Found {} todos", todos.len());
/// ```
pub fn read() -> Result<Vec<Todo>, ActaError> {
    let file_path = init()?;

    let serialized_list = fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&serialized_list)?)
}

/// Writes todo items to the default storage file
///
/// This function serializes and writes the provided todo list to the storage file,
/// replacing any existing content. The file will be created if it doesn't exist.
///
/// # Arguments
///
/// * `todos` - A slice of `Todo` items to persist
///
/// # Returns
///
/// Returns `Ok(())` on successful write.
///
/// # Errors
///
/// - `ActaError::HomeDir` if the home directory cannot be determined
/// - `ActaError::Parse` if the todos cannot be serialized to JSON
/// - `ActaError::Io` if the file cannot be written
///
/// # Examples
///
/// ```no_run
/// use acta::persistence::write;
/// use acta::model::Todo;
///
/// let todos = vec![/* ... */];
/// write(&todos).expect("Failed to write todos");
/// ```
pub fn write(todos: &[Todo]) -> Result<(), ActaError> {
    let file_path = init()?;

    let serialized_list = serde_json::to_string(&todos)?;
    fs::write(file_path, serialized_list)?;

    Ok(())
}
