
## Workspace structure

The workspace is organized into the following crates.

### `api`

This crate contains all server functions. To differentate between frontent usage (where function definitions are converted into `reqwest` clients) and backend usage (where function definitions are converted into `axum` handlers) the crate uses the `backend` feature. This feature pulls in traits from the `backend` crate using the `backend` `prelude`.

This split makes it a lot more convinent to work with server-only dependencies which any server-function using crate would otherwise need to declare as optional and then enabled on e.g. a `server` feature.

### `backend`

This crate contains pure backend functionality such as `axum` middleware, service definitions (as traits) and the `main` method for starting the backend.

### `frontend`

This crate contains shared / most of the UI code (which is used in turn by the `web` and `mobile` crates) and the `main` method for starting the frontend.

### `shared`

This crate contains plain shared data structures (such as `Session`) which are used (or returned) by `api` functions for usage in the `frontend`, `mobile` and `web` crates.

### Platform crates: `web` and `mobile`

These crates contain the web and mobile app code (and in the future native code). They use the `frontend` crate for 99% of their styling and logic.
