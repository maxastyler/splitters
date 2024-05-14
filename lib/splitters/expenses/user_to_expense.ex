defmodule Splitters.Expenses.UserToExpense do
  use Ecto.Schema
  import Ecto.Changeset

  schema "user_to_expense" do
    field :proportion_paid, :integer
    field :user_id, :id
    field :expense_id, :id

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(user_to_expense, attrs) do
    user_to_expense
    |> cast(attrs, [:proportion_paid])
    |> validate_required([:proportion_paid])
  end
end
