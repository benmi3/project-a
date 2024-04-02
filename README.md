# Time keeping thingy currently named project-a

I want to expand my knowledge of both rust and general software design.
This is why I chose to use an app blue print from [rust10x](https://rust10x.com/web-app).
This is a blue print for production level application, so I hope that working with this will help expanding my knowledge for software design.

## The goal for this project

### Projects

I want to try to track my time use, devided up in projects.
One project might be as an example, general work at x workplace, or this project at this workplace.

### Time Records

I want to then add timerecords of what I did where. I named to place for now, but I would at travel to x workplace when travling back and forth.

### Tasks

I want to add all tasks I did.

### Task Progress

From 0-100, adding a prosentage to each task for when I did what. 
My main focus is not how fast I did the task, but rather progress went.
The reason for this is to see later, if I see that progress is slow on x type of thing, maybe I need to learn more about it,
or find a better way to do x type of thing.

## Contributions

Feel free to open pull requests, or create Issues of feature requests. 
The main goal is to learn on the way, so getting some guidance on the way is appreciated.






---

# Rust10x Web App Blueprint for Production Coding

More info at: https://rust10x.com/web-app

## Rust10x Web App YouTube Videos:

- [Episode 01 - Rust Web App - Base Production Code](https://youtube.com/watch?v=3cA_mk4vdWY&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
    - [Topic video - Code clean -  `#[cfg_attr(...)]` for unit test](https://www.youtube.com/watch?v=DCPs5VRTK-U&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [Topic video - The Reasoning Behind Differentiating ModelControllers and ModelManager](https://www.youtube.com/watch?v=JdLi69mWIIE&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [Topic video - Base64Url - Understanding the Usage and Significance of Base64URL](https://www.youtube.com/watch?v=-9K7zNgsbP0&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- [Episode 02 - Sea-Query (sql builder) & modql (mongodb like filter)](https://www.youtube.com/watch?v=-dMH9UiwKqg&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- [Episode 03 - Cargo Workspace (multi-crates)](https://www.youtube.com/watch?v=zUxF0kvydJs&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)
	- [AI-Voice-Remastered](https://www.youtube.com/watch?v=iCGIqEWWTcA&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- [Episode 04 - Multi-Scheme Password Hashing](https://www.youtube.com/watch?v=3E0zK5h9zEs&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

- Other Related videos: 
	- [Rust Axum Full Course](https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)


## Starting the DB

```sh
# Start postgresql server docker image:
podman run --rm --name pg -p 5432:5432 \
   -e POSTGRES_PASSWORD=welcome \
   postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
podman exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

## Dev (watch)

> NOTE: Install cargo watch with `cargo install cargo-watch`.

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w crates/services/web-server/src/ -w crates/libs/ -w .cargo/ -x "run -p web-server"

# Terminal 2 - To run the quick_dev.
cargo watch -q -c -w crates/services/web-server/examples/ -x "run -p web-server --example quick_dev"
```

## Dev

```sh
# Terminal 1 - To run the server.
cargo run -p web-server

# Terminal 2 - To run the tests.
cargo run -p web-server --example quick_dev
```

## Unit Test (watch)

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter.
cargo watch -q -c -x "test -p lib-core test_create -- --nocapture"
```

## Unit Test

```sh
cargo test -- --nocapture

cargo watch -q -c -x "test -p lib-core model::task::tests::test_create -- --nocapture"
```

## Tools

```sh
cargo run -p gen-key
```

<br />

---

More resources for [Rust for Production Coding](https://rust10x.com)


[This repo on GitHub](https://github.com/rust10x/rust-web-app)
