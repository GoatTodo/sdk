use crate::todo::Todo;

pub mod memory;

/// Defines functions a storage client must implement.
/// The StorageClient should be expected to be used
/// in Client, the lifetimes match, and any data source should
/// be dropped / closed when StorageClient goes out of scope.
///
/// In the future we could switch this to match the public
/// sdk model where we have storage client types. For now,
/// we define a bunch of functions for internal simplicity.
/// Example:
/// ```
/// // let sc = MemoryStorageClient::new();
///
/// // sc.todo_create( {todo stuff here} );    // current way
/// // sc.todos().create( {todo stuff here} ); // future way
/// ```
pub trait StorageClient {
    /// Create a new StorageClient
    fn new() -> Self;
    /// Create a todo
    fn todo_create(&mut self, todo: Todo) -> Result<(), ()>;
    /// Get a list of all todos
    fn todo_list(&self) -> Result<Vec<Todo>, ()>;
    /// Delete all todo data
    fn todo_dump(&mut self) -> Result<(), ()>;
    //TODO: fn todo_update(&mut self, todo: Todo) -> Result<(), ()>;
    //TODO: fn todo_delete(&mut self, todo: Todo) -> Result<(), ()>;
    // TODO: handle the drop stuff
}
