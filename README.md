# SpiritFingers

SimHash NIFs written in Rust.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `spirit_fingers` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:spirit_fingers, "~> 0.1.0"}
  ]
end
```

## Usage

Generate SimHash:
``` elixir
SimHash.simhash("wow")
{:ok, 17399923637769257768}
```

Hamming Distance between hashes:
```
SimHash.hamming_distance(17399923637769257768, 17399923637769257768)
{:ok, 0.0}
```

Similarity between hashes:
```
SimHash.hash_similarity(17399923637769257768, 17399923637769257768)
{:ok, 0.0}
```

Similarity between strings:
```
SimHash.similarity("Hocus pocus", "Hocus pocus pilatus pas")
{:ok, 0.9375}
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/spirit_fingers](https://hexdocs.pm/spirit_fingers).

