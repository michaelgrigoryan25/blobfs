# stormi

Stormi is a fast and simple file-server with public/private key authentication

# How does it work?

Stormi accepts `multipart/form-data` form with media payload, then it writes the data to the disk and automatically infers the mimetype of the uploaded files.

Whenever you want to remove files, you'll need to supply the hashes in a JSON form which will have the following structure

```json
{
  "hashes": [
    "hash-1",
    "hash-2", 
    "hash-3"
  ]
}
```

There is no need to specify file suffix since it will be automatically matched by Stormi using the [`glob`](https://docs.rs/glob/0.3.0/glob/) crate(e.g.`hash-123.png` will be matched as `hash-123.*`)
