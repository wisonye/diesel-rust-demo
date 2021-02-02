# `diesel` Rust Demo

## Start and stop docker container

```bash
# Start testing container
docker-compose up --detach

# Stop testing container
docker-compse down -t5
```

## Init database

After running the `docker-compose up --deatch` for the first time, run the command
below to init the database:

```bash
diesel migration run
```

And then open `src/schema.rs` and change the `id -> Int4,` to `id -> Nullable<Int4>,`
which looks like below:

```rust
table! {
    users (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        is_male -> Bool,
    }
}
```

That makes the auto increased primary id become `Option<i32>` type in `Rust`.

## How to run test bin

- Add users

    ```bash
    cargo run --bin add_users
    # Plz type your new user name here:
    # Benny Chen
    # 
    # Saved users: User {
    #     id: Some(
    #         6,
    #     ),
    #     name: "Benny Chen",
    #     is_male: true,
    # }
    ```

    </br>

- Show all added users

    ```bash
    cargo run --bin show_users
    # User {
    #     id: Some(
    #         4,
    #     ),
    #     name: "Rober Ma",
    #     is_male: true,
    # }
    # -----------
    # 
    # User {
    #     id: Some(
    #         5,
    #     ),
    #     name: "Andy France",
    #     is_male: true,
    # }
    # -----------
    ```
