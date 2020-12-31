defmodule PasswordCracker do

  @spec find_valid_passwords(Range.t()) :: [integer()]
  def find_valid_passwords(range) do
    range
    |> Enum.map(&is_password_valid?/1)
    |> Enum.filter(fn x -> x == true end)
  end

  @spec find_valid_passwords(Range.t()) :: [integer()]
  def find_truly_valid_passwords(range) do
    range
    |> Enum.map(&is_password_truly_valid?/1)
    |> Enum.filter(fn x -> x == true end)
  end

  @spec is_password_valid?(integer()) :: boolean()
  def is_password_valid?(password) do
    [&has_required_digit_count?/1, &has_non_decreasing_digits?/1, &has_same_adjacent_digits?/1]
    |> run_validators(password)
  end

  @spec is_password_truly_valid?(integer()) :: boolean()
  def is_password_truly_valid?(password) do
    [&has_required_digit_count?/1, &has_non_decreasing_digits?/1, &has_exactly_two_adjacent_digits?/1]
    |> run_validators(password)
  end

  defp run_validators(validators, password) do
    validators
    |> Enum.map(fn x -> x.(password) end)
    |> Enum.reduce(&Kernel.and/2)
  end

  defp has_required_digit_count?(password) do
    Enum.count(Integer.digits(password)) == 6
  end

  defp has_non_decreasing_digits?(password) do
    Integer.digits(password) == Enum.sort(Integer.digits(password))
  end

  defp has_same_adjacent_digits?(password) do
    Integer.digits(password)
    |> Enum.chunk_by(fn x -> x end)
    |> Enum.map(&Enum.count/1)
    |> Enum.map(fn x -> x >= 2 end)
    |> Enum.reduce(&Kernel.or/2)
  end

  defp has_exactly_two_adjacent_digits?(password) do
    Integer.digits(password)
    |> Enum.chunk_by(fn x -> x end)
    |> Enum.map(&Enum.count/1)
    |> Enum.map(fn x -> x == 2 end)
    |> Enum.reduce(&Kernel.or/2)
  end
end

defmodule ProblemSolver do

  def solve1() do
    load_input_range("input.txt")
    |> PasswordCracker.find_valid_passwords
    |> Enum.count()
  end

  def solve2() do
    load_input_range("input.txt")
    |> PasswordCracker.find_truly_valid_passwords
    |> Enum.count()
  end

  @spec load_input_range(String.t()) :: Range.t()
  defp load_input_range(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.trim
    |> String.split("-")
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple
    |> tuple_to_range
  end

  @spec tuple_to_range({integer(), integer()}) :: Range.t()
  defp tuple_to_range({range_start, range_end}), do: range_start..range_end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
