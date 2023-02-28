# rust-new-project-template
A good starting point for a new Rust project


## install mini-redis
`cargo install mini-redis` 

## start the server
`mini-redis-server`

### note: Since async will return a Future, we also need to use .await to make the Future run, and finally get the return value.
