defmodule Fuel do
  def formula(x), do: div(x, 3) - 2

  def calculator(current_block, sum) when current_block <= 0, do: sum - current_block
  def calculator(current_block, sum) do
    new_addition = Fuel.formula(current_block)
    Fuel.calculator(new_addition, sum + new_addition)
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_modules("input.txt")
    |> Enum.map(&Fuel.formula/1)
    |> Enum.sum
  end

  def solve2() do
    load_modules("input.txt")
    |> Enum.map(&Fuel.calculator(&1, 0))
    |> Enum.sum
  end

  defp load_modules(filename) do
    File.stream!(filename)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
