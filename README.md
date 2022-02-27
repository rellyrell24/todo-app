## Chapter 2: Designing Your Web Application in Rust

1. What does the `--release` argument in Cargo do when added to build and run
    * `cargo build --release`
        - uses defaults for release build
        - opt-level = 3, spends more time compiling
    * `cargo run --release`
        - run optimized artifacts with the release profile
2. How do we enable a file to be accessible within and outside a module?
    - Define the file as module in the `mod.rs` file in the root of the module. Add `pub` before the definition to make it accessible out the module
3. What are the advantages of having a trait with a single scope
    - single-scope traits enable maximum flexibility when defining structs
4. What steps would we have to add an `on hold` to-do item that will only allow get and edit functionality?
    1. Define a trait that inherits the base struct, which implements the get and edit traits
    2. Add another option in the todo factory
    3. Add another entrypoint for on hold items
5. What are the benefits of the factory?
    - standardizes the construction of structs
    - reduces the possibility of building one of a range of structs outside of the module with just one line of code
    - stops other files ballooning 
    - does not require the developer to root around in the module to utilize it
6. How do we effectively map a range of processes on some parameters?
    - Use `match` statements that lead on to other `match` statements

## Chapter 4: Processing HTTP Requests

1. What is the difference between a GET request and POST request?
    * `GET`: cached, limitation to payload size
    * `POST`: request with body, not cached, more payload than `GET`
2. Why would we have middleware when we check credentials?
    - To open the header and check the credentials before sending the request to the desired view
    - To prevent the body being loaded by returning an auth error before loading the view
3. How do you enable custom struct to be directly returned in a view?
    - Implement the `Responder` trait
    - Define the `responded_to` function
4. How do you enact middleware for the server?
    - Must enact the `wrap_fn` function on the `App` struct
    - Pass closure that accepts the service request and routing structs
5. How do you enable struct to serialize data into the view?
    - Use `#[derive(Deserialize)]` macro
    - Define the parameter type so that it's wrapped in a JSON struct