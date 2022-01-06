# stormi

Stormi is a fast and simple, hash-based file-server written in Rust

## How does it work?

Stormi accepts `multipart/form-data` form with media payload, writes the data to disk and automatically infers the mimetype of the uploaded files.

When `GET`ting files Stormi uses a glob search with a given `hash` and wildcard(`*`) extension match(`hash.*`).

![Diagram 1](./docs/images/diagram-1.png)

### Endpoints

Stormi has 3 endpoints:

- `/:hash` - **GET** - For getting a file with its hash
- `/upload` - **POST** - For uploading a set of files
- `/remove` - **POST** - For removing items from Stormi

### Request structure

- #### Request structure for `/remove` endpoint

```json
{
  "hashes": ["hash1", "hash2", "hash3"]
}
```

- ##### Request to `/upload` should contain a `multipart/form-data` form

#### Send a request to `/upload` with `curl`

Create a **base64 encoded** string with your credentials in the following format: `username:password`, replace `credentials` with the output and change `file.png` to the filename you want to upload.

Send the request `curl`:

```shell
curl -X POST --form file='file.png' http://127.0.0.1:6345/upload -H 'Authorization: credentials'
```

### Response structure

Response structure of `/upload` and `/remove` is the same:

```json
{
  "skipped": [],
  "hashes": []
}
```

- `skipped` - the hases that have been skipped
- `hashes` - the hashes that have been added, modified or removed

### Wrappers

Here is a list of different wrapper implementations around the Stormi API.

- [TypeScript](https://github.com/polygon-isecure/stormi.ts)

#### Connection string structure

When connecting to Stormi via wrappers, it will be helpful to utilize a
connection string instead of supplying all the parameters in separate fields.
Stormi connection string follows the structure below:

```http
http://<user>:<pass>@<host>:<port>/
```

In the real world, you would have a connection string that looks similar to this

```http
https://admin:str0ng-passw0rd@48.247.139.178:6345/
```

### Configuring and running Stormi

To configure Stormi, you will need to create a `config.yaml` file and add at least **one** user with its password and permissions to `users`. Check out the [configuration example](./examples/config.example.yaml).

You can compile Stormi locally using the following command:

```shell
cargo build --release
```

and start it using:

```shell
./target/release/stormi
```

#### Deploy with Docker

You can also create a Stormi instance using Docker

Pull the image:

```shell
docker pull ghcr.io/polygon-isecure/stormi:latest
```

Create an instance:

```shell
docker run -dp 6435:6435 --name stormi ghcr.io/polygon-isecure/stormi:latest
```
