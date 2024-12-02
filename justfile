# https://vinnymeller.com/posts/rust-webserver-hot-reload/
run:
  cargo watch -q -c -w src/ -w .cargo/ -x 'run --bin rust-web-app'

test:
  cargo watch -q -c -w tests/ -x 'test -q quick_dev -- --nocapture'

quick_dev:
  cargo watch -q -c -w examples/ -x 'run --example quick_dev'

postgres:
  docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

psql:
  docker exec -it -u postgres pg psql
