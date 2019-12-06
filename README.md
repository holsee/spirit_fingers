# SpiritFingers

"Fast SimHash NIFs written in Rust 🐇💨 as Erlang/Elixir versions were too slow 🐢"

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

* Elixir ~> 1.9
* OTP 22
* Rust 2018 ~> 1.39 `(4560ea788 2019-11-04)`
* Rustler 1.21.0

## Installation

Add `spirit_fingers` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:spirit_fingers, "~> 0.3.0"}
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

