defmodule SpiritFingers.MixProject do
  use Mix.Project

  def project do
    [
      app: :spirit_fingers,
      version: "0.5.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases(),
      dialyzer: dialyzer(),
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
      {:rustler, "~> 0.37.1"},
      {:ex_doc, "~> 0.34", only: :dev, runtime: false},
      {:dialyxir, "~> 1.4", only: [:dev, :test], runtime: false},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false}
    ]
  end

  defp package() do
    [
      name: "spirit_fingers",
      files: [
        "config",
        "lib",
        "native/simhash/src",
        "native/simhash/Cargo.toml",
        "native/simhash/Cargo.lock",
        "mix.exs",
        "README.md",
        "LICENSE",
        "logo.png"
      ],
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

  defp dialyzer do
    [
      plt_add_apps: [:mix],
      ignore_warnings: ".dialyzer_ignore.exs"
    ]
  end

  defp aliases do
    [
      "test.all": ["test", "test.rust"]
    ]
  end

  def cli do
    [
      preferred_envs: [
        "test.all": :test,
        "test.rust": :test
      ]
    ]
  end
end
