# D(e)ad Jokes API

This is a RESTful API that serves up a collection of (e)ad jokes.
The API is built using Rust and Cargo.

## Getting Started

To get started with the API, you'll need to have Rust and Cargo installed
on your system. Once you have those installed, you can clone the repository
and run the following command to start the server:

```bash
cargo run
```

By default, the server will listen on port 4343. You can change the port
by setting the `API_PORT` environment variable.

## Endpoints

The API has the following endpoints:

- `GET /api/jokes/random`: Returns a random joke.
- `GET /api/jokes`: Returns a list of all jokes with pagination info.
- `GET /api/jokes/{id}`: Returns a single joke by ID.
- `POST /api/jokes`: Adds a new joke to the collection.
- `PUT /api/jokes/{id}`: Updates an existing joke.

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
