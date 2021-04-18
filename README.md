# shurl

Simple URL shortener

### Usage

```bash
# generate a short link with expiration time
curl -XPOST \
     -H "Content-Type: application/json" \
     -d '{"url": "https://doc.rust-lang.org/book/title-page.html", "expired_at": "2021-12-31T23:59:59Z"}' \
     http://localhost:8080/api/links

# generate a perpetual short link
curl -XPOST \
     -H "Content-Type: application/json" \
     -d '{"url": "https://doc.rust-lang.org/book/title-page.html", "expired_at": null}' \
     http://localhost:8080/api/links
```
