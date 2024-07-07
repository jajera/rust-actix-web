# rust-actix-web

cargo init

cargo add actix-web

cargo add uuid --features=v4

cargo add lazy_static

cargo clean

## test

docker buildx build .

docker run <imageid> -p 8080:8080

docker buildx build -t rust_actix_web . && docker run --rm -p 8080:8080 rust_actix_web
