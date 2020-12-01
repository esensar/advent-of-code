defmodule Entries do

  def find_sumpair(entries, sum) do
    entries
    |> Enum.map(fn x -> sum - x end)
    |> Enum.filter(fn x -> Enum.count(entries, fn y -> x == y end) > 0 end)
  end

  def find_sumtriple(entries, sum) do
    entries
    |> Enum.map(fn x -> {x, find_sumpair(entries, sum - x)} end)
    |> Enum.filter(fn {_, sumpair} -> Enum.count(sumpair) > 0 end)
    |> Enum.map(fn {x, sumpair} -> sumpair ++ [x] end)
    |> Enum.at(0)
  end

  def find_sumpair_product(entries, sum) do
      find_sumpair(entries, sum)
      |> Enum.reduce(fn x, acc -> x * acc end)
  end

  def find_sumtriple_product(entries, sum) do
      find_sumtriple(entries, sum)
      |> Enum.reduce(fn x, acc -> x * acc end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_entries("input.txt")
    |> Entries.find_sumpair_product(2020)
  end

  def solve2() do
    load_entries("input.txt")
    |> Entries.find_sumtriple_product(2020)
  end

  defp load_entries(filename) do
    File.stream!("2020/1/#{filename}")
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
