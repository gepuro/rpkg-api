# How to use API.

search R language package.
http://rpkg-api.gepuro.net/rpkg?q={word}

# Build

```
docker build -t rpkg-api:latest .
```

# Run

docker run -d --rm -p 8000:8000 -v data:/opt/rpkg-api/data rpkg-api:latest

Access http://localhost:8000/rpkg .
