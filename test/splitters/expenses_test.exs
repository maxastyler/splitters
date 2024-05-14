defmodule Splitters.ExpensesTest do
  use Splitters.DataCase

  alias Splitters.Expenses

  describe "currencies" do
    alias Splitters.Expenses.Currency

    import Splitters.ExpensesFixtures

    @invalid_attrs %{name: nil}

    test "list_currencies/0 returns all currencies" do
      currency = currency_fixture()
      assert Expenses.list_currencies() == [currency]
    end

    test "get_currency!/1 returns the currency with given id" do
      currency = currency_fixture()
      assert Expenses.get_currency!(currency.id) == currency
    end

    test "create_currency/1 with valid data creates a currency" do
      valid_attrs = %{name: "some name"}

      assert {:ok, %Currency{} = currency} = Expenses.create_currency(valid_attrs)
      assert currency.name == "some name"
    end

    test "create_currency/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Expenses.create_currency(@invalid_attrs)
    end

    test "update_currency/2 with valid data updates the currency" do
      currency = currency_fixture()
      update_attrs = %{name: "some updated name"}

      assert {:ok, %Currency{} = currency} = Expenses.update_currency(currency, update_attrs)
      assert currency.name == "some updated name"
    end

    test "update_currency/2 with invalid data returns error changeset" do
      currency = currency_fixture()
      assert {:error, %Ecto.Changeset{}} = Expenses.update_currency(currency, @invalid_attrs)
      assert currency == Expenses.get_currency!(currency.id)
    end

    test "delete_currency/1 deletes the currency" do
      currency = currency_fixture()
      assert {:ok, %Currency{}} = Expenses.delete_currency(currency)
      assert_raise Ecto.NoResultsError, fn -> Expenses.get_currency!(currency.id) end
    end

    test "change_currency/1 returns a currency changeset" do
      currency = currency_fixture()
      assert %Ecto.Changeset{} = Expenses.change_currency(currency)
    end
  end

  describe "expenses" do
    alias Splitters.Expenses.Expense

    import Splitters.ExpensesFixtures

    @invalid_attrs %{amount: nil, date: nil, description: nil}

    test "list_expenses/0 returns all expenses" do
      expense = expense_fixture()
      assert Expenses.list_expenses() == [expense]
    end

    test "get_expense!/1 returns the expense with given id" do
      expense = expense_fixture()
      assert Expenses.get_expense!(expense.id) == expense
    end

    test "create_expense/1 with valid data creates a expense" do
      valid_attrs = %{amount: 42, date: ~U[2024-05-13 08:09:00Z], description: "some description"}

      assert {:ok, %Expense{} = expense} = Expenses.create_expense(valid_attrs)
      assert expense.amount == 42
      assert expense.date == ~U[2024-05-13 08:09:00Z]
      assert expense.description == "some description"
    end

    test "create_expense/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Expenses.create_expense(@invalid_attrs)
    end

    test "update_expense/2 with valid data updates the expense" do
      expense = expense_fixture()
      update_attrs = %{amount: 43, date: ~U[2024-05-14 08:09:00Z], description: "some updated description"}

      assert {:ok, %Expense{} = expense} = Expenses.update_expense(expense, update_attrs)
      assert expense.amount == 43
      assert expense.date == ~U[2024-05-14 08:09:00Z]
      assert expense.description == "some updated description"
    end

    test "update_expense/2 with invalid data returns error changeset" do
      expense = expense_fixture()
      assert {:error, %Ecto.Changeset{}} = Expenses.update_expense(expense, @invalid_attrs)
      assert expense == Expenses.get_expense!(expense.id)
    end

    test "delete_expense/1 deletes the expense" do
      expense = expense_fixture()
      assert {:ok, %Expense{}} = Expenses.delete_expense(expense)
      assert_raise Ecto.NoResultsError, fn -> Expenses.get_expense!(expense.id) end
    end

    test "change_expense/1 returns a expense changeset" do
      expense = expense_fixture()
      assert %Ecto.Changeset{} = Expenses.change_expense(expense)
    end
  end
end
