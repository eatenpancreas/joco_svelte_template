
![Joco Svelte Template](./static/Joco-01.png)
# -- TRAPS stack --

<table>
  <tr>
    <td>T</td>
    <td>R</td>
    <td>A</td>
    <td>P</td>
    <td>S</td>
  </tr>
  <tr>
    <td>Typescript</td>
    <td>Rust</td>
    <td>Actix</td>
    <td>Postgres</td>
    <td>Svelte</td>
  </tr>
  <tr>
    <td>Tailwind</td>
    <td></td>
    <td></td>
    <td></td>
    <td>Sqlx</td>
  </tr>
  <tr>
    <td>Ts-rs</td>
    <td></td>
    <td></td>
    <td></td>
    <td>Shadcn</td>
  </tr>

</table>

TRAPS focuses on runtime safety, DX, UX, performance and scalability. It leverages the strength
of Rust and Typescript to provide a robust and secure backend and frontend.

## FEATURES
- [x] Secure HTTP-Only-Cookie-based JWT authentication built-in
- [x] Typesafe and secure SQL queries with sqlx
- [x] Secure back-end with Actix-web
- [x] Pretty and swiftly built frontends thanks to Svelte, Tailwind and Shadcn
- [x] Automatically generated API bindings in Typescript using a in-house macro in combination with `ts-rs`
---
## STARTING OUT

Make sure you have a postgres DB running, copy `env.example` 
to `.env` and set the properties accordingly. 
Also make sure to set a `env.production` file.

The `compose.example.yaml` file is a template for the docker-compose 
file. Make sure to use the same passwords as defined in the `.env` file.

### Backend
* Run `cargo sqlx database create` to create the database.
* Run `cargo sqlx migrate run` to run the migrations.
* Run `cargo run` to start the backend server.

### Frontend
* Run `npm install` to install the dependencies.
* Run `npm run dev` to start the development server.

### Dev
* Run `cargo test` to test, this also runs migrations and generates/updates the API bindings.
* Run `npm run test` to test the frontend.
* Check out `package.json` for more scripts. Most of the listed commands have been aliased here.

---
## FOLDER STRUCTURE

* `.` - Contains the frontend and the cargo workspace
  * `api` - The backend
  * `api-lib` - The backend library
  * `api-proc` - Some procedural macros
  * `src` - Sveltekit frontend
    * `/api` - The generated API bindings, accessible by `$api` in TS or Svelte files
  * `static` - Static files
  * `tests` - Integration tests


## Deployment

Make sure you have a postgres DB running, and have a valid `.env.production` according to 
the `.env.example` file. Also make sure to have a valid `compose.yaml` file.

For docker, unfortunately postgres needs to be running already when the backend starts.
This is due to the fact that the backend is started before the database is created and SQLX
tries to connect to the database. This is a known issue and will be fixed in the future.


## Todos

- [ ] Add more tests
- [ ] Add more documentation
- [ ] Email authentication
- [ ] Built-in admin panel
- [ ] Resend email verification & Add a way to change email / password
- [ ] Error handling in the frontend
- [ ] Github actions for CI/CD and fixing the docker-compose issue