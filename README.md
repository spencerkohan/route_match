# Route Match

A utility for simple and intuitve routing of http requests using macros.

## Quick Example

With `route_match`, you can route a request like this:

```rust
use route_match::route;

fn handle_request(reqest: Request) -> Response {
  route! {
    match (&reqest.method().as_str(), &reqest.uri().path()) {
      GET /foo/bar => handle_foo_bar(),
      POST /user/:id => handle_post_user(request, id),
      _ => handle_404_error(request),
    }
  }
}
```

## Goals

HTTP routing should be easy.  The HTTP protocol hit version 1.0 in 1996 - it's an incredibly well-trodden technology, and it should be a well-solved problem by now to route a request based on a URL and HTTP method, and extract variables from a URL path.

In my experience, **existing routing soutions** often encounter several pitfalls which make this harder than it has to be:

- They're often overly **coupled to a single solution**.  One library might have a great routing solution, but you have to adopt other components of their chosen networking stack if you want to use it.  It's not always easy if you want to move your API to a different context, like a cloud function, where you're constrained to use a different Request and Response type.

- They're often magical, and due to heavy use of traits, can be **hard to understand**, can give confusing error messages, and can take time to learn the "magic words" to make them work.

- They often **come with limitations**.  When you need to squeeze your solution through a routing interface interface, you might encounter challenges in dealing with lifetimes, or async which were not anticipated or designed around by the library author.

- They often **come with a runtime cost**.  The router object will often be represented as an object on the heap which has to be iterated over.  Admittedly this is a very very minor one, but it doesn't have to be that way.

So `route_match` aims to solve these problems, and provide a ***simple-to-use, intuitive, context-agnostic*** solution for routing HTTP requests in Rust.

The priorities of this project are:

1. To provide an easy to use, intuitive DSL for routing HTTP requests, through a procedural macro

2. To let you think about your endpoints in terms of the HTTP protocol rather than Rust syntax

3. To work in any context where you have an http method and a URL path, regardless of the other tools or the stack you are working with

4. To focus on one use-case, and to do it well.  This libary does not attempt to solve every possible routing scenario under the sun, it aims to handle the most common scenarios while remaining lean, minimal, fast to download and fast to build.


## Usage

Here's an example of a simple match statement:

```rust
fn match_route(method: &str, path: &str) -> boolean {
    route! {
      match (method, path) {
        GET /foo => true,
        _ => false,
      }
    }
}

match_route("GET", "/foo") // returns true
match_route("GET", "foo") // also true - the leading '/' is optional
match_route("POST", "/foo") // returns false - the method must match
match_route("GET", "/bar") // returns false - the path must match
match_route("GET", "/foo/bar") // returns false - the uri path must match completely
```

The provided method and path will be checked in order, and the first matching branch will be executed.  So for instance for a match expression like this:

```rust
fn match_route(method: &str, path: &str) {
    route! {
      match (method, path) {
        _ /foo -> println!("ANY /foo"),
        GET /foo => println!("GET /foo"),
        _ => println!("default"),
      }
    }
}

match_route("GET", "/foo") // prints "ANY /foo"
```

the `GET /foo` condition will never be executed, because `_ /foo` matches the `GET` condition as well.

Also note, the default `_` branch must always be provided.

### URL Path Parameters

We can also extract url parameters by inserting the pattern `:var_name` in the path pattern:

```rust
fn match_route(method: &str, path: &str) -> boolean {
    route! {
      match (method, path) {
        GET /user/:id => println!("user id: {}", id),
        _ => println!("default"),
      }
    }
}

match_route("GET /user/456") // prints: "GET /user/456"
```

Here the `id` parameter is passed to the branch expression, as an `&str`.  The lifetime of the parameter is the same as the lifetime of the `path` argument which is passed to the match expression.

### Wildcard matches

Sometimes we want to ignore part of a pattern and match inclusively

```rust
fn match_route(method: &str, path: &str) -> boolean {
    route! {
      match (method, path) {
        // Match any method, so long as the path matches "/foo"
        _ /foo => println!("Any /foo"),
        // Match any request with the method "OPTIONS"
        OPTIONS _ => println!("OPTIONS"),
        // Mathc any path starting with "/foo/bar"
        // Here "rest" will be bound as an &str containing everything
        // in the path following "/foo/bar"
        GET /foo/bar/..:rest => println("rest: {}", rest),
        // Match any method/path combination at all
        _ => println!("default"),
      }
    }
}
```

## Grammar

The `route` macro provides a match expression, which lets you match against HTTP methods and uri patterns.

The match statement takes the form:

> match_stmnt : `match` `(` <method> `,` <path> `)` `{` <branches> `}`
> method: *Expression*
> path: *Expression*

Here the `method` and `path` arguments can be any expression which has the type `&str`.

`branches` expands to the following:

> branches : <branch>, <branches>? | <branch>?
> branch : <pattern> => *Expression*
>
> pattern : <method> <uri> | _
> method : `GET` | `HEAD` | `POST` | `PUT` | `DELETE` | `CONNECT` | `OPTIONS` | `TRACE` | `PATCH` | `_`
> uri : <uri_components> | `"` <uri_components> `"` | `_`

> uri_components : `/` <uri_component> <uri_components>? | `/` <uri_component>?
> uri_component : IDENTIFIER | <named_var> | <rest_component>
> named_var : `:` IDENTIFIER
> rest_component : `..` <named_vat>?

## Runtime Specification

At runtime, the match statement executes the first branch expression, such that the method and path provided match the branch pattern.
