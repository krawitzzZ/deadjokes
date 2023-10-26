# D(e)ad Jokes API

This is a RESTful API that serves up a collection of (e)ad jokes.
The API is built using Rust and Cargo.

## Getting Started

To get started with the API, you'll need to have Rust (1.73) and Cargo installed
on your system. Once you have those installed, you can clone the repository
and run the following command to check that you can build the server:

```bash
make
```

Once you have verified that you can build the server, you can start with
setting up the infrastructure. The API uses PostgreSQL as a database and
ELK stack for logging. In order to simplify the setup, you can copy the
`.env.example` file to `.env` and edit it to match your environment.
Reasonable defaults are provided.

Once you have the `.env` file set up, you can run the following command:

```bash
make infra
```

This will start the database and the logging stack. Once the infrastructure
containers are up and running you will need to crate an index for the logs:

```bash
.data/infra/init-elastic.sh
```

This should be enough to get you started. You can now run the server:

```bash
make run
```

**Note**: for the initial database seed dropbox client is used to download
`/jokes.sql` seed file. If you want to seed the database you will need to
define `DBX_OAUTH_TOKEN` (long living token) variable in the `.env`.  If you
don't have the seed file or don't want to seed the database you can skip this
step by providing `DB_SKIP_SEED` environment variable: `DB_SKIP_SEED=true`.

If you want to run the server entirely in Docker, you can use the following
commands:

```bash
make image # build the api image
make up
```

Before running the server in Docker, you will need to adjust the `.env` file
accordingly, since the hosts will be different for infra services.

## Endpoints

The API has the following endpoints:

- `GET /api/status`: Returns ok if API is up and running.
- `GET /api/jokes/random`: Returns a random joke.
- `GET /api/jokes`: Returns a list of all jokes with pagination info.
- `GET /api/jokes/{id}`: Returns a single joke by ID.
- `POST /api/jokes`: Adds a new joke to the collection. Payload is a JSON
  object with `body` field containing the joke text.
- `PUT /api/jokes/{id}`: Updates an existing joke. Payload the same as for
  `POST /api/jokes`.

## Data Model

The API uses the following data model:

```json
{
  "id": "97b22197-d027-41c7-9a05-447ed6c13e91",
  "body": "What is the blood type of people who are poor at grammar? Typo",
  "createdAt": "2023-10-22T16:36:48.055458Z",
  "lastUpdatedAt": "2023-10-22T16:36:48.055458Z"
}
```

## Notes

Please note, that jokes in the collection are not original.
They were taken from the various sources on the internet, therefore
I do not claim any ownership of the jokes.
Jokes may contain offensive language.

## License

This project is licensed under the MIT License.
See the [LICENSE](./LICENSE) file for details.
