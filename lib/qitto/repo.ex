defmodule Qitto.Repo do
  use Ecto.Repo,
    otp_app: :qitto,
    adapter: Ecto.Adapters.SQLite3
end
