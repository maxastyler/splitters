defmodule Splitters.Repo.Migrations.CreateExpenses do
  use Ecto.Migration

  def change do
    create table(:expenses) do
      add :description, :string
      add :amount, :integer, null: false

      timestamps(type: :utc_datetime)
    end
  end
end
