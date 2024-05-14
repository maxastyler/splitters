defmodule Splitters.Expenses.Expense do
  use Ecto.Schema
  import Ecto.Changeset

  schema "expenses" do
    field :amount, :integer
    field :date, :utc_datetime
    field :description, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(expense, attrs) do
    expense
    |> cast(attrs, [:description, :amount, :date])
    |> validate_required([:description, :amount, :date])
  end
end
