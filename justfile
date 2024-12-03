# https://vinnymeller.com/posts/rust-webserver-hot-reload/
run:
  # cargo watch -q -c -w src/ -w .cargo/ -x 'run --bin rust-web-app'
  cargo watch -q -c -w crates/services/web-server/src/ -w crates/libs/ -w .cargo/ -x "run -p web-server"

quick_dev:
  # cargo watch -q -c -w examples/ -x 'run --example quick_dev'
  cargo watch -q -c -w crates/services/web-server/examples/ -x "run -p web-server --example quick_dev"

postgres:
  docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

psql:
  docker exec -it -u postgres pg psql

test:
  # cargo watch -q -c -w tests/ -x 'test -q quick_dev -- --nocapture'
 
  cargo watch -q -c -x "test -- --nocapture"
  # cargo watch -q -c -x "test model::task::tests::test_delete -- --nocapture"

