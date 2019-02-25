# SpiritFingers

"Fast SimHash NIFs written in Rust 🐇💨 as Erlang/Elixir versions were too slow 🐢"

![logo](./logo.png)

* [Hex Package](https://hex.pm/packages/spirit_fingers).
* [Documentation](https://hexdocs.pm/spirit_fingers).

## Versions

* Elixir ~> 1.8
* OTP 21
* Rust 2018 ~> 1.32 `(9fda7c223 2019-01-16)`
* Rustler 1.19.1

## Build

```
mix compile
```

## Test

```
mix test
```

## Installation

Add `spirit_fingers` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:spirit_fingers, "~> 0.2"}
  ]
end
```

## Usage

Generate SimHash:
``` elixir
SimHash.simhash("wow")
{:ok, 17399923637769257768}
```

Similarity between strings:
``` elixir
SimHash.similarity("Hocus pocus", "Hocus pocus pilatus pas")
{:ok, 0.9375}
```

Hamming Distance between hashes:
``` elixir
SimHash.hamming_distance(17399923637769257768, 17399923637769257768)
{:ok, 0.0}
```

Similarity between hashes:
``` elixir
SimHash.hash_similarity(17399923637769257768, 17399923637769257768)
{:ok, 0.0}
```

