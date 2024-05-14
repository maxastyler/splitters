defmodule Splitters.Repo.Migrations.CreateCurrencies do
  use Ecto.Migration

  def change do
    create table(:currencies) do
      add :name, :string, null: false

      timestamps(type: :utc_datetime)
    end

    create unique_index(:currencies, [:name])
  end
end
