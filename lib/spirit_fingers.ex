defmodule SimHash do
  use Rustler, otp_app: :spirit_fingers, crate: "simhash"

  @doc """
  ## Examples
    iex> SimHash.simhash("The cat sat on the mat")
    {:ok, 2595200813813010837}

    iex> SimHash.simhash("The cat sat under the mat")
    {:ok, 2595269945604666783}

    iex> SimHash.simhash("Why the lucky stiff")
    {:ok, 1155526875459215761}
  """
  def simhash(_bin), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  ## Examples
    iex> SimHash.hamming_distance(0, 0)
    {:ok, 0.0}

    iex> SimHash.hamming_distance(0b1111111, 0b0000000)
    {:ok, 7.0}

    iex> SimHash.hamming_distance(0b0100101, 0b1100110)
    {:ok, 3.0}
  """
  def hamming_distance(_hash0, _hash1), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  ## Examples
    iex> SimHash.hash_similarity(0, 0)
    {:ok, 1.0}

    iex> SimHash.hash_similarity(0xFFFFFFFFFFFFFFFF, 0)
    {:ok, 0.0}

    iex> SimHash.hash_similarity(0xFFFFFFFF, 0)
    {:ok, 0.5}
  """
  def hash_similarity(_hash0, _hash1), do: :erlang.nif_error(:nif_not_loaded)


  @doc """

  ## Examples
    iex> SimHash.similarity("Stop hammertime", "Stop hammertime")
    {:ok, 1.0}

    iex> SimHash.similarity("Hocus pocus", "Hocus pocus pilatus pas")
    {:ok, 0.9375}

    iex> SimHash.similarity("Peanut butter", "Strawberry cocktail")
    {:ok, 0.59375}
  """
  def similarity(_text0, _text1), do: :erlang.nif_error(:nif_not_loaded)


end
