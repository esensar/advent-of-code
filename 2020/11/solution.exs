defmodule SeatMap do
  @enforce_keys [:width, :height, :lines]
  defstruct [:width, :height, :lines]

  def stabilize(seatmap) do
    seatmap
    |> apply_rules(&apply_rule/3)
  end

  def stabilize_with_new_rules(seatmap) do
    seatmap
    |> apply_rules(&apply_new_rule/3)
  end

  defp apply_rules(seatmap, rule_func) do
    table = :ets.new(:cache, [:set, :protected])
    :ets.insert(table, {"count", 0})
    new_lines = seatmap.lines
                |> Enum.with_index
                |> Enum.map(fn {row, rownum} -> 
                  row 
                  |> Enum.with_index
                  |> Enum.map(fn {col, colnum} ->
                    new_col = rule_func.(seatmap, rownum, colnum)
                    if col != new_col do
                      [{_, current_count} | _] = :ets.lookup(table, "count")
                      :ets.insert(table, {"count", current_count + 1})
                    end
                    new_col
                  end)
                end)

    new_map = %SeatMap{
      width: seatmap.width,
      height: seatmap.height,
      lines: new_lines
    }
    [{_, changes} | _] = :ets.lookup(table, "count")
    if changes != 0, do: apply_rules(new_map, rule_func), else: new_map
  end

  defp apply_rule(seatmap, row, col) do
    seat = Enum.at(seatmap.lines, row) |> Enum.at(col)

    case seat do
      "L" -> if count_adjacent_occupied(seatmap, row, col) == 0, do: "#", else: "L"
      "#" -> if count_adjacent_occupied(seatmap, row, col) >= 4, do: "L", else: "#"
      x -> x
    end
  end

  defp apply_new_rule(seatmap, row, col) do
    seat = Enum.at(seatmap.lines, row) |> Enum.at(col)

    case seat do
      "L" -> if count_visibly_occupied(seatmap, row, col) == 0, do: "#", else: "L"
      "#" -> if count_visibly_occupied(seatmap, row, col) >= 5, do: "L", else: "#"
      x -> x
    end
  end

  defp count_adjacent_occupied(seatmap, row, col) do
  -1..1
  |> Enum.flat_map(fn r -> 
  -1..1
  |> Enum.map(fn c -> {r, c} end)
  end)
  |> Enum.filter(fn {r, c} -> r != 0 || c != 0 end)
  |> Enum.count(fn {r, c} -> is_occupied?(seatmap, row + r, col + c) end)
  end

  defp count_visibly_occupied(seatmap, row, col) do
  -1..1
  |> Enum.flat_map(fn r -> 
  -1..1
  |> Enum.map(fn c -> {r, c} end)
  end)
  |> Enum.filter(fn {r, c} -> r != 0 || c != 0 end)
  |> Enum.count(fn {r, c} -> is_direction_occupied?(seatmap, row, col, r, c) end)
  end

  defp is_direction_occupied?(seatmap, row, col, dir_row, dir_col, mod \\ 1) do
    calculated_row = row + dir_row * mod
    calculated_col = col + dir_col * mod
    cond do
      calculated_row >= seatmap.height or calculated_row < 0 -> false
      calculated_col >= seatmap.width or calculated_col < 0 -> false
      Enum.at(seatmap.lines, calculated_row) |> Enum.at(calculated_col) == "." -> is_direction_occupied?(seatmap, row, col, dir_row, dir_col, mod + 1)
      true -> Enum.at(seatmap.lines, calculated_row) |> Enum.at(calculated_col) == "#"
    end
  end

  defp is_occupied?(seatmap, row, col) do
    cond do
      row >= seatmap.height or row < 0 -> false
      col >= seatmap.width or col < 0 -> false
      true -> Enum.at(seatmap.lines, row) |> Enum.at(col) == "#"
    end
  end

  def parse_map(lines) do
    %SeatMap{
      width: Enum.at(lines, 0)
      |> String.graphemes
      |> Enum.count(),
      height: Enum.count(lines),
      lines: Enum.map(lines, &String.graphemes/1)
    }
  end
end

defmodule ProblemSolver do
  def solve1() do
    stabilized_map = load_map("input.txt")
    |> SeatMap.stabilize

    stabilized_map.lines
    |> Enum.flat_map(fn x -> x end)
    |> Enum.count(fn x -> x == "#" end)
  end

  def solve2() do
    stabilized_map = load_map("input.txt")
    |> SeatMap.stabilize_with_new_rules

    stabilized_map.lines
    |> Enum.flat_map(fn x -> x end)
    |> Enum.count(fn x -> x == "#" end)
  end

  defp load_map(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> SeatMap.parse_map
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
