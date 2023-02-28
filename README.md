# rust-new-project-template
A good starting point for a new Rust project

![Figure](https://github.com/nogibjj/week6-rust/blob/main/Screen%20Shot%202023-02-28%20at%204.27.33%20PM.png)

## install mini-redis
`cargo install mini-redis` 

## start the server
`mini-redis-server`

### note: Since async will return a Future, we also need to use .await to make the Future run, and finally get the return value.
