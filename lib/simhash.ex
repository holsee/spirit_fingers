defmodule SpiritFingers.SimHash do
  @moduledoc """
  SimHash Module which delegates to Rust NIFs which will
  perform the hashing, similarity and distance calculations.
  """
  use Rustler, otp_app: :spirit_fingers, crate: "simhash"

  @typedoc "unsigned 64 bit integer represenation of simhash"
  @type t :: pos_integer()

  @typedoc """
  Similarity between two `SimHash.t`, represented as a value
  between 0.0 and 1.0.
  * `0.0` means no similarity,
  * `1.0` means identical.
  """
  @type similarity :: float()

  @typedoc """
  64 bit floating point represenation of the
  [Hamming Distance](https://en.wikipedia.org/wiki/Hamming_distance)
  between 2 `SimHash.t`.
  """
  @type distance :: float()

  @doc """
  Calculate `SimHash.t` split by whitespace.

  ## Examples

      iex> SpiritFingers.SimHash.simhash("The cat sat on the mat")
      {:ok, 2595200813813010837}

      iex> SpiritFingers.SimHash.simhash("The cat sat under the mat")
      {:ok, 2595269945604666783}

      iex> SpiritFingers.SimHash.simhash("Why the lucky stiff")
      {:ok, 1155526875459215761}
  """
  @spec simhash(binary()) :: t()
  def simhash(_bin), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Bitwise hamming distance of two `SimHash.t` hashes

  ## Examples

      iex> SpiritFingers.SimHash.hamming_distance(0, 0)
      {:ok, 0.0}

      iex> SpiritFingers.SimHash.hamming_distance(0b1111111, 0b0000000)
      {:ok, 7.0}

      iex> SpiritFingers.SimHash.hamming_distance(0b0100101, 0b1100110)
      {:ok, 3.0}
  """
  @spec hamming_distance(t(), t()) :: distance()
  def hamming_distance(_hash0, _hash1), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Calculate similarity as `SimHash.similarity` of two hashes.
  `0.0` means no similarity, `1.0` means identical.

  ## Examples

      iex> SpiritFingers.SimHash.hash_similarity(0, 0)
      {:ok, 1.0}

      iex> SpiritFingers.SimHash.hash_similarity(0xFFFFFFFFFFFFFFFF, 0)
      {:ok, 0.0}

      iex> SpiritFingers.SimHash.hash_similarity(0xFFFFFFFF, 0)
      {:ok, 0.5}
  """
  @spec hash_similarity(t(), t()) :: similarity()
  def hash_similarity(_hash0, _hash1), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Calculate similarity `SimHash.similarity` of two string slices split by whitespace by simhash.

  ## Examples

      iex> SpiritFingers.SimHash.similarity("Stop hammertime", "Stop hammertime")
      {:ok, 1.0}

      iex> SpiritFingers.SimHash.similarity("Hocus pocus", "Hocus pocus pilatus pas")
      {:ok, 0.9375}

      iex> SpiritFingers.SimHash.similarity("Peanut butter", "Strawberry cocktail")
      {:ok, 0.59375}
  """
  @spec similarity(binary(), binary()) :: similarity()
  def similarity(_text0, _text1), do: :erlang.nif_error(:nif_not_loaded)
end
