defmodule SpiritFingersTest do
  use ExUnit.Case
  doctest SpiritFingers

  test "greets the world" do
    assert SpiritFingers.hello() == :world
  end
end
