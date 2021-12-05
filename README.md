# stormi

Stormi is a fast and simple, hash-based file-server written in Rust

## How does it work?

Stormi accepts `multipart/form-data` form with media payload, then it writes the data to the disk and automatically infers the mimetype of the uploaded files.

## Endpoints

Stormi has 3 endpoints:

- `/:hash` - **GET** - For getting a file with its hash
- `/upload` - **POST** - For uploading a set of files
- `/remove` - **POST** - For removing items from Stormi

## Request structure

- Request structure for `/remove` endpoint:

```json
{
  "hashes": [
    "hash1",
    "hash2",
    "hash3"
  ]
}
```

- Request to `/upload` should be `multipart/form-data`

## Response structure

Response structure of `/upload` and `/remove` is the same:

```json
{
  "hashes": [],
  "skipped": []
}
```

- `hashes` - the hashes that have been added, modified or removed
- `skipped` - the hases that have been skipped

## Wrappers

Here is a list of different wrapper implementations around the Stormi API

- [TypeScript](https://github.com/polygon-isecure/stormi.ts)

## Configuring and running Stormi

To configure Stormi, you will need to create a `config.yaml` file and add a `key` value, which will be the secret "token" to access Stormi.

```yaml
key: "some random stuff"
```

You can compile Stormi locally using the following command:

```shell
$ cargo build --release
```

and start it using:

```shell
$ ./target/release/stormi
```

## Docker

You can also create a Stormi instance using Docker

Pull the image:

```docker
docker pull ghcr.io/polygon-isecure/stormi:latest
```

Create an instance:

```docker
docker run -dp 6435:6435 --name stormi ghcr.io/polygon-isecure/stormi:latest
```
