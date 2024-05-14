defmodule Splitters.ExpensesFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Splitters.Expenses` context.
  """

  @doc """
  Generate a unique currency name.
  """
  def unique_currency_name, do: "some name#{System.unique_integer([:positive])}"

  @doc """
  Generate a currency.
  """
  def currency_fixture(attrs \\ %{}) do
    {:ok, currency} =
      attrs
      |> Enum.into(%{
        name: unique_currency_name()
      })
      |> Splitters.Expenses.create_currency()

    currency
  end

  @doc """
  Generate a expense.
  """
  def expense_fixture(attrs \\ %{}) do
    {:ok, expense} =
      attrs
      |> Enum.into(%{
        amount: 42,
        date: ~U[2024-05-13 08:09:00Z],
        description: "some description"
      })
      |> Splitters.Expenses.create_expense()

    expense
  end
end
