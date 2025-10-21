defmodule Mix.Tasks.Test.Rust do
  @moduledoc """
  Runs Rust unit tests for the native simhash crate.

  ## Usage

      mix test.rust

  This task runs `cargo test` in the native/simhash directory.
  """
  @shortdoc "Run Rust unit tests"

  use Mix.Task

  @impl Mix.Task
  def run(_args) do
    crate_path = Path.join([File.cwd!(), "native", "simhash"])

    if File.dir?(crate_path) do
      Mix.shell().info("Running Rust tests in #{crate_path}...")

      case System.cmd("cargo", ["test"], cd: crate_path, into: IO.stream(:stdio, :line)) do
        {_, 0} ->
          Mix.shell().info("\nRust tests passed! ✓")
          :ok

        {_, exit_code} ->
          Mix.shell().error("\nRust tests failed with exit code #{exit_code}")
          Mix.raise("Rust tests failed")
      end
    else
      Mix.raise("Rust crate directory not found at #{crate_path}")
    end
  end
end
