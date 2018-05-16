defmodule SimHash do
  use Rustler, otp_app: :spirit_fingers, crate: "simhash"

  def simhash(_bin), do: :erlang.nif_error(:nif_not_loaded)

  def hamming_distance(hash0, hash1), do: :erlang.nif_error(:nif_not_loaded)

  def hash_similarity(hash0, hash1), do: :erlang.nif_error(:nif_not_loaded)
end
