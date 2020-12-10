defmodule JoltageAdapters do

  def find_ways_to_connect_adapter(adapters) do
    :ets.new(:cache, [:named_table])
    adapters
    |> Enum.sort
    |> count_distinct_valid_connections
  end

  defp count_distinct_valid_connections(adapters, current_val \\ 0) do
    cached = :ets.lookup(:cache, current_val)
    if Enum.count(cached) > 0 do
      Enum.at(cached, 0)
      |> elem(1)
    else
      if Enum.count(adapters) == 0 or Enum.at(adapters, 0) - current_val > 3 do
        1
      else
        # Only possible jumps may be found in first 3 elements
        result = Enum.take(adapters, 3)
        |> Enum.with_index
        |> Enum.reduce_while(0, fn {x, index}, acc ->
          if x - current_val <= 3 do
            {:cont, acc + count_distinct_valid_connections(Enum.slice(adapters, (index + 1)..Enum.count(adapters)), Enum.at(adapters, index))}
          else
            {:halt, acc}
          end
        end)
        :ets.insert(:cache, {current_val, result})
        result
      end
    end
  end

  def find_joltage_diff_groups(adapters) do
    adapters
    |> Enum.sort
    |> Enum.reduce_while({0, 0, 1, 0}, fn x, {j1, j2, j3, last} = acc ->
      case (x - last) do
        1 -> {:cont, {j1 + 1, j2, j3, x}}
        2 -> {:cont, {j1, j2 + 1, j3, x}}
        3 -> {:cont, {j1, j2, j3 + 1, x}}
        _ -> {:halt, acc}
      end
    end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    {j1, _, j3, _} = load_adapters("input.txt")
                     |> JoltageAdapters.find_joltage_diff_groups
    j1 * j3
  end

  def solve2() do
    load_adapters("input.txt")
    |> JoltageAdapters.find_ways_to_connect_adapter
  end

  defp load_adapters(filename) do
    File.stream!(Path.expand(filename, __DIR__))
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
