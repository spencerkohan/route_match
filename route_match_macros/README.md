# Route Macros

This is a procedural macro for implementing routing in hyper.

## Usage

```rust

route! {
    match request {
        GET   /foo/bar => get_fubar(request)
        POST   /foo/:id => post_foo(request, id)
    }
}

```

This expands to:

```rust

  {
      let method = request.method.clone();
      let path: Vec<&str> = request.path.clone().split('/').collect();
      if let Some(()) = {
          if &method != &http::Method::GET {
              None
          } else if path.len() != PATH_LENGTH {
              None
          } else if path[0] != "foo" || path[1] != "bar" {
              None
          } else {
              Some(())
          }
      } {
          get_fubar(request)
      } else if let Some(id) = {
          if &method != &http::Method::POST {
              None
          } else if path.len() != PATH_LENGTH {
              None
          } else if path[0] != "foo" {
              None
          } else {
              let id = path[1];
              Some(id)
          }
      } {
          post_foo(request, id)
      }

  }

```
