defmodule BinarySeat do
  @seatcode %{"F" => "0", "B" => "1", "L" => "0", "R" => "1"}

  def seat_id({row, column}), do: row * 8 + column

  def parse_pass(pass) do
    pass = String.replace(pass, ~r/./, fn x -> @seatcode[x] end)
    row = pass
          |> String.slice(0, 7)
          |> String.to_integer(2)

    column = pass
          |> String.slice(7, 3)
          |> String.to_integer(2)
    {row, column}
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_passes("input.txt")
    |> Enum.map(&BinarySeat.parse_pass/1)
    |> Enum.map(&BinarySeat.seat_id/1)
    |> Enum.max()
  end

  def solve2() do
    all_ids = load_passes("input.txt")
    |> Enum.map(&BinarySeat.parse_pass/1)
    |> Enum.map(&BinarySeat.seat_id/1)

    max_id = Enum.max(all_ids)
    min_id = Enum.min(all_ids)
    Enum.at(Enum.to_list(min_id..max_id) -- all_ids, 0)
  end

  defp load_passes(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
