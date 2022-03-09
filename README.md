# sysc4806project

CircleCI Link: https://app.circleci.com/pipelines/github/Eliasin/sysc4806project?invite=true

Heroku Link: https://sysc4806-rust.herokuapp.com/

## Deploying locally

```
Set up Postgres database
    - https://www.postgresql.org/docs/9.0/sql-createdatabase.html

cargo install diesel

diesel migration run --database-url DATABASE_URL

use Rocket.example.toml to create and configure Rocket.toml

cargo run
```

The application is then accessible at http://localhost:8000/