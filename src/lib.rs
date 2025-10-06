mod clients;
mod timestamp;
mod todo;
mod user;

pub fn hello_world() -> String {
    String::from("Hello world!")
}

/*
    Goat Todo SDK

    Potential API Brainstorming

    - GoatTodoClient -

    let client = GoatTodoClient.new();
    client.login(user)
    client.logout()

    client.add_todo
    client.get_todo
    client.update_todo
    client.delete_todo

    - Todo -

    ?

    The client represents the storage abstraction for todo's.

    let client = Client::new(cloud | filesystem | memory | local db);
    client.login(user);
    client.logout(user);

    User -> email & password
*/
