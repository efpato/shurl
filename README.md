# shurl

Simple URL shortener

### Usage

```bash
# generate a short link with lifetime 1h
curl -XPOST \
     -H "Content-Type: application/json" \
     -d '{"url": "https://doc.rust-lang.org/book/title-page.html", "keep_sec": 3600}' \
     http://localhost:8080

# generate a permanent short link
curl -XPOST \
     -H "Content-Type: application/json" \
     -d '{"url": "https://doc.rust-lang.org/book/title-page.html"}' \
     http://localhost:8080
```

### Development environment
```bash
docker-compose up -d
diesel setup
```
