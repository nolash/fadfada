# Fadafada

Fadafada is a network request multiplexer.

It allows the user to define a prioritized sequence of network endpoints to request against. The requests may also be offset in time.

When a single endpoint returns a result, all other active requests are by default cancelled.


## Use cases

Fadafada is especially designed for developers and applications tha wish to gradually transition to permissionless decentralized storage.

It allows actual integration with web3 storage, while still keeping the other foot in the world of web2.


## The cast

- **timing**: A simple structure defining a time offset and timeout.
- **resolver**: Translates an endpoint reference from one endpoint type to another.
- **controller**: Graph defining groups of valid endpoints and their timings.
- **group**: A group of endpoints of the same type, sharing the same resolver.
- **endpoint**: The network service queried for the resource.
- **adapter**: A unit that knows how to request from a specific endpoint type.
- **request**: A single request made to a single endpoint.


## Resolving resources

In its initial form, all endpoints will be queried for the same resource identifier.

If different resource identifiers exist for the same resource, individual groups of endpoints may define their own resolver to perform the necessary reference translation.


## Example

Image `I` is looked up on two endpoint `group`s:

- A regular web2 HTTP server, using the image's `sha256` sum as identifier `A`
- A Swarm network, using the image's `swarmhash` sum as identifier `B` 

`I` has three known mirrored web2 locations, `W1`, `W2` and `W3`.

The client additionally knows that the resource exists on [Swarm](https://ethswarm.org), and also knows of two Swarm nodes that allow file requests, `S1` and `S2`.

The client wants to try, in sequence:

1. The Swarm service
2. A web2 fallback
3. The second Swarm service
4. The remaining two web2 services.

First, the client defines a `resolver`, who knows how to translate the identifier `A` to identifer `B`. This resolver is connected to the Swarm `adapter`. 

The client defines a `controller` graph as follows:

1. A `group` of two Swarm `endpoint`s, with a `timing`0.5 seconds apart.
2. A `group` of three web2 `endpoint`s, with a `timing` 0.2 seconds apart, starting at 0.4 seconds after the first Swarm `request`.

This results in the following individual `request`s being made, at the given time _offsets_:

1. 0.0s - Swarm request for `A` to `S1`
1. 0.4s - Web2 request for `B` to `W1`
1. 0.5s - Swarm request for `A` to `S2`
1. 0.6s - Web2 request for `B` to `W2`
1. 0.8s - Web2 request for `B` to `W3`


## Extensions


### Quorom requests

At least `n` matching results must be returned from any `n` endpoints. The remaining active requests will be cancelled.


### Persisting requests for benchmarking

Response stats (time, volume) are stored to a chosen backend.

None, some or all requests remaining after the request normally would have been fulfilled are always completed.


