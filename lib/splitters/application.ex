defmodule Splitters.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      SplittersWeb.Telemetry,
      Splitters.Repo,
      {DNSCluster, query: Application.get_env(:splitters, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Splitters.PubSub},
      # Start the Finch HTTP client for sending emails
      {Finch, name: Splitters.Finch},
      # Start a worker by calling: Splitters.Worker.start_link(arg)
      # {Splitters.Worker, arg},
      # Start to serve requests, typically the last entry
      SplittersWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Splitters.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    SplittersWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
