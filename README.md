# sysc4806project

CircleCI Link: https://app.circleci.com/pipelines/github/Eliasin/sysc4806project?invite=true

**This is for backend, Do not click to view frontend:**
Heroku Link: https://sysc4806-rust.herokuapp.com/

Frontend Link: https://sysc4806project-frontend.vercel.app/

Frontend Git: https://github.com/Eliasin/sysc4806project-frontend

Database Diagram and Schema are in this Repository

## Deploying locally

```
Set up Postgres database
    - https://www.postgresql.org/docs/9.0/sql-createdatabase.html

Install diesel
    - https://diesel.rs/guides/getting-started

diesel migration run --database-url DATABASE_URL

Use Rocket.example.toml to create and configure Rocket.toml

cargo run
```

The application is then accessible at http://localhost:8000/

## Plans for upcoming iteration

```
- Add research topic creation/deletion/modification.
- Finish implementing professor editing and research fields
- Implement application accepting and rejecting
- Add email notifications using SendGrid
- Improve UI look and feel
```
