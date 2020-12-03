defmodule TobboganMap do
  @enforce_keys [:width, :height, :lines]
  defstruct [:width, :height, :lines]

  def count_trees_on_angle(map, right, down, current_right \\ 0, current_down \\ 0, tree_count \\ 0)

  def count_trees_on_angle(%TobboganMap{height: height} = _map, _right, _down, _current_right, current_down, tree_count) when current_down >= height do
    tree_count
  end

  def count_trees_on_angle(map, right, down, current_right, current_down, tree_count) do
    tree_count + tree_inc(map, current_right, current_down) + count_trees_on_angle(map, right, down, current_right + right, current_down + down, tree_count)
  end

  defp tree?(map, right, down) do
    Enum.at(map.lines, down)
    |> Enum.at(rem(right, map.width))
    |> Kernel.==("#")
  end

  defp tree_inc(map, right, down) do
    if tree?(map, right, down), do: 1, else: 0
  end

  def parse_map(lines) do
    %TobboganMap{
      width: Enum.at(lines, 0)
      |> String.graphemes
      |> Enum.count(),
      height: Enum.count(lines),
      lines: lines |> Enum.map(&String.graphemes/1)
    }
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_map("input.txt")
    |> TobboganMap.count_trees_on_angle(3, 1)
  end

  def solve2() do
    map = load_map("input.txt")
    [{1, 1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}]
    |> Enum.map(fn {right, down} -> TobboganMap.count_trees_on_angle(map, right, down) end)
    |> Enum.reduce(fn x, acc -> x * acc end)
  end

  defp load_map(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> TobboganMap.parse_map
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
