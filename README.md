[![Rust build status](https://img.shields.io/github/actions/workflow/status/travisbrown/twitter-model/ci.yaml?branch=main)](https://github.com/travisbrown/twitter-model/actions)

# Twitter API model

This repository contain [JSON Schema][json-schema] definitions for several types of data returned by the [Twitter API][twitter-api],
together with a Rust library for working with Twitter API data.

Please note that this software is **not** "open source",
but the source is available for use and modification by individuals, non-profit organizations, and worker-owned businesses
(see the [license section](#license) below for details).

## Scope

This project does not attempt to provide schemas for all Twitter API types, and currently only supports a subset of the v1.1 API.
It is intended to validate every status object in the Internet Archive's [Twitter Stream Grab][twitter-stream-grab] (which spans
about a decade of API results). (It doesn't currently meet this goal, but should soon.)

## Status

The following types are currently supported:

* Results from the [v1.1 search API](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/search/api-reference/get-search-tweets)
* User objects from the [v1.1 `users/lookup` endpoint](https://developer.twitter.com/en/docs/twitter-api/v1/tweets/search/api-reference/get-search-tweets) (with several extended parameters)
* A large sample of Twitter Stream Grab status objects from 2011 and 2017

## Approach

One goal of the project is to represent _all_ of the data in the Twitter Stream Grab.
All object types in the schema have `additionalProperties` set to `false`.

The Rust library uses [Typify][typify] for code generation. The generated code is currently checked into version control,
but this may change in the future.

## License

This software is published under the [Anti-Capitalist Software License][acsl] (v. 1.4).

[acsl]: https://anticapitalist.software/
[json-schema]: https://json-schema.org/
[twitter-api]: https://developer.twitter.com/en/docs/twitter-api
[twitter-stream-grab]: https://archive.org/details/twitterstream
[typify]: https://github.com/oxidecomputer/typify