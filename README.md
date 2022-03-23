# sysc4806project

CircleCI Link: https://app.circleci.com/pipelines/github/Eliasin/sysc4806project?invite=true

Heroku Link: https://sysc4806-rust.herokuapp.com/

Frontend Link: https://sysc4806project-frontend.vercel.app/

Frontend Git: https://github.com/Eliasin/sysc4806project-frontend

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
