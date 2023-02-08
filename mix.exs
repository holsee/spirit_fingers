defmodule SpiritFingers.MixProject do
  use Mix.Project

  def project do
    [
      app: :spirit_fingers,
      version: "0.4.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      name: "SpiritFingers",
      source_url: "https://github.com/holsee/spirit_fingers",
      homepage_url: "https://hex.pm/packages/spirit_fingers",
      docs: [main: "SpiritFingers", logo: "logo.png", extras: ["README.md"]],
      package: package(),
      description: description()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.27.0"},
      {:ex_doc, "~> 0.29.1", only: :dev, runtime: false},
      {:dialyxir, "~> 1.2", only: [:dev], runtime: false}
    ]
  end

  defp package() do
    [
      name: "spirit_fingers",
      files: ["config", "lib", "native", "mix.exs", "README*", "LICENSE*"],
      links: %{
        "GitHub" => "https://github.com/holsee/spirit_fingers"
      },
      maintainers: ["Steven Holdsworth (@holsee)"],
      licenses: ["MIT"]
    ]
  end

  defp description() do
    "Fast SimHash NIFs written in Rust 🐇💨 as Erlang/Elixir versions were too slow 🐢"
  end
end
