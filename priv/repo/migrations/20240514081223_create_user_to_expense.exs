defmodule Splitters.Repo.Migrations.CreateUserToExpense do
  use Ecto.Migration

  def change do
    create table(:user_to_expense, primary_key: false) do
      add :proportion_paid, :integer
      add :user_id, references(:users, on_delete: :nothing), primary_key: true
      add :expense_id, references(:expenses, on_delete: :nothing), primary_key: true

      timestamps(type: :utc_datetime)
    end

    create index(:user_to_expense, [:user_id])
    create index(:user_to_expense, [:expense_id])
  end
end
