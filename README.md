# SpiritFingers

"Fast SimHash NIFs written in Rust 🐇💨 as Erlang/Elixir versions were too slow 🐢"

For a full comparison versus native elixir solutions see: [Simhash Benchmarks](https://github.com/holsee/simhash_benchmarks)
TL;DR Spirit Fingers is 400-900x faster, orders of magnitude more memory efficient and handles large binaries where others cannot. 

![logo](./logo.png)

* [Hex Package](https://hex.pm/packages/spirit_fingers).
* [Documentation](https://hexdocs.pm/spirit_fingers).

## Build

```
mix compile
```

## Test

```
mix test
```

## Versions

* Elixir ~> 1.14
* OTP 25
* Rust 2021 ~> 1.67.0
* Rustler 0.27.0

## Installation

Add `spirit_fingers` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:spirit_fingers, "~> 0.4.0"}
  ]
end
```

## Usage

Generate SimHash:
``` elixir
SpiritFingers.SimHash.similarity_hash("wow")
{:ok, 17399923637769257768}
```

Similarity between strings:
``` elixir
SpiritFingers.SimHash.similarity("Hocus pocus", "Hocus pocus pilatus pas")
{:ok, 0.9375}
```

Hamming Distance between hashes:
``` elixir
SpiritFingers.SimHash.hamming_distance(17399923637769257768, 17399923637769257768)
{:ok, 0.0}
```

Similarity between hashes:
``` elixir
SpiritFingers.SimHash.hash_similarity(17399923637769257768, 17399923637769257768)
{:ok, 1.0}
```

