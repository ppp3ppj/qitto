defmodule Qitto.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    pubsub = System.get_env("ELIXIRKIT_PUBSUB")

    children = [
      QittoWeb.Telemetry,
      Qitto.Repo,
      {Ecto.Migrator,
       repos: Application.fetch_env!(:qitto, :ecto_repos), skip: skip_migrations?()},
      {DNSCluster, query: Application.get_env(:qitto, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Qitto.PubSub},
      # ElixirKit PubSub bridge — connects to Tauri when launched from desktop app
      {ElixirKit.PubSub,
       connect: pubsub || :ignore,
       on_exit: fn -> System.stop() end},
      QittoWeb.Endpoint,
      # Notify Tauri that Phoenix + Ecto are ready — open window
      {Task,
       fn ->
         if pubsub do
           ElixirKit.PubSub.broadcast("messages", "ready")
         end
       end}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Qitto.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    QittoWeb.Endpoint.config_change(changed, removed)
    :ok
  end

  defp skip_migrations?() do
    # By default, sqlite migrations are run when using a release
    System.get_env("RELEASE_NAME") == nil
  end
end
