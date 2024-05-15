defmodule Splitters.Expenses.Expense do
  use Ecto.Schema
  import Ecto.Changeset

  schema "expenses" do
    field :amount, :integer
    field :description, :string

    many_to_many :users, Splitters.Accounts.User, join_through: "user_to_expense"

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(expense, attrs) do
    expense
    |> cast(attrs, [:description, :amount])
    |> validate_required([:description, :amount])
    |> cast_assoc(:users, required: true)

  end
end
