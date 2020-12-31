defmodule Customs do

  def parse_group_answers_all_yes(group) do
    parse_person_answers(group)
    |> Enum.reduce(&MapSet.intersection/2)
  end

  def parse_group_answers(group) do
    parse_person_answers(group)
    |> Enum.reduce(&MapSet.union/2)
  end

  defp parse_person_answers(group) do
    group
    |> String.split(~r/\n/)
    |> Enum.map(&String.graphemes/1)
    |> Enum.map(&MapSet.new/1)
    |> Enum.filter(fn x -> Enum.count(x) != 0 end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_groups("input.txt")
    |> Enum.map(&Customs.parse_group_answers/1)
    |> Enum.map(&Enum.count/1)
    |> Enum.sum
  end

  def solve2() do
    load_groups("input.txt")
    |> Enum.map(&Customs.parse_group_answers_all_yes/1)
    |> Enum.map(&Enum.count/1)
    |> Enum.sum
  end

  defp load_groups(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.split("\n\n")
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
