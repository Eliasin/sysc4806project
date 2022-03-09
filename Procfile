web: ROCKET_PORT=$PORT ROCKET_ADDRESS=0.0.0.0 ROCKET_DATABASES="{db = { url = \"$DATABASE_URL\", pool_size = 10 }}" ./target/release/sysc4806_project
release: ./target/release/diesel migration run