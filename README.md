# `diesel` Rust Demo

## Start and stop docker container

```bash
# Start testing container
docker-compose up --detach

# Stop testing container
docker-compse down -t5
```

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
