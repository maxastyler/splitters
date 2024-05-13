defmodule Splitters.Repo do
  use Ecto.Repo,
    otp_app: :splitters,
    adapter: Ecto.Adapters.Postgres
end
