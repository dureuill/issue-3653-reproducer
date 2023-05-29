# issue-3653-reproducer
This repository is a minimal reproducer for actix web's behavior that leads to https://github.com/meilisearch/meilisearch/issues/3653.

It does not uses Meilisearch, rather, it starts the actix runtime from a background thread. From the main thread, it waits for the actix runtime to be up and listening, and then it makes multiple requests from many threads.

- When the requests are made to a "fast" route ("hello"), then they all succeed with HTTP 200.
- When the requests are made to a route that is artificially slow with a thread sleep ("sleepy-hello"), then most of them fail with HTTP 408 client timeout.

There's little that actix could do to recover if a user of the library has a route that takes too much time synchronously, but the return code shouldn't be a HTTP 4xx. It should be a 5xx, as this is coming from the server, not the client.

