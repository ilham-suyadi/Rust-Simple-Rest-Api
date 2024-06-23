
### Get
```bash
curl localhost:8080/api/v1/blogs
```

### Single Get
``` bash
curl localhost:8080/api/v1/blog/2
```

### Post
```bash
curl -X POST 127.0.0.1:8080/api/v1/blog -H "Content-Type: application/json" -d '{"id":3, "title":"rust docs", "content":"lorem ipsum"}'
```

### Edit
```bash
curl -X PUT 127.0.0.1:8080/api/v1/blog/3 -i -H "Content-Type: application/json" -d '{"id":2, "title":"Rust Docs v2", "content":"lorem ipsum dolor simit"}'
```

### Detele
```bash
curl -X DELETE 127.0.0.1:8080/api/v1/blog/3
```