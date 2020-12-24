defmodule HexTiler do
  @directions_map %{
    "e" => {2, 0},
    "w" => {-2, 0},
    "sw" => {-1, -1},
    "nw" => {-1, 1},
    "se" => {1, -1},
    "ne" => {1, 1}
  }

  def load_initial_state(directions) do
    directions
    |> Enum.map(&directions_to_coordinates/1)
    |> Enum.group_by(fn x -> x end)
    |> Enum.map(fn {group, items} -> {group, item_count_to_color(items)} end)
    |> Map.new
  end

  def count(state, target_color) do
    state
    |> Enum.count(fn {_, color} -> color == target_color end)
  end

  def run_for(state, days) when days == 0, do: state
  def run_for(state, days) do
    state
    # First add white tiles as neighbors if they don't exist
    |> Enum.flat_map(fn x ->
      [x | generate_missing_neighbors(state, x)]
    end)
    # Then go flipping
    |> Enum.map(fn x ->
      flip_daily(state, x)
    end)
    |> Map.new
    |> run_for(days - 1)
  end

  defp generate_missing_neighbors(state, {{tile_x, tile_y}, _}) do
    @directions_map
    |> Enum.map(fn {_, {x, y}} ->
      {tile_x + x, tile_y + y}
    end)
    |> Enum.filter(fn x -> Map.has_key?(state, x) == false end)
    |> Enum.map(fn x -> {x, :white} end)
  end

  defp flip_daily(state, {tile_coords, tile_color} = tile) do
    if tile_color == :black do
      neighbor_count = count_neighbors(state, tile, :black)
      if neighbor_count == 0 or neighbor_count > 2, do: {tile_coords, :white}, else: tile
    else
      if count_neighbors(state, tile, :black) == 2, do: {tile_coords, :black}, else: tile
    end
  end

  defp count_neighbors(state, {{tile_x, tile_y}, _}, target_color) do
    @directions_map
    |> Enum.map(fn {_, {x, y}} ->
      {tile_x + x, tile_y + y}
    end)
    |> Enum.map(&Map.get(state, &1))
    |> Enum.count(fn color -> color == target_color end)
  end

  defp item_count_to_color(items) do
    if rem(Enum.count(items), 2) != 0, do: :black, else: :white
  end

  defp directions_to_coordinates(directions_line) do
    directions_line
    |> String.graphemes
    |> Enum.chunk_while(
      [],
      fn x, acc ->
        if x in ["n", "s"] do
          {:cont, acc ++ [x]}
        else
          {:cont, acc ++ [x], []}
        end
      end,
      fn
        [] -> {:cont, []}
        acc -> {:cont, acc, []}
      end
    )
    |> Enum.map(&Enum.join/1)
    |> Enum.map(&Map.get(@directions_map, &1))
    |> Enum.reduce(fn {x, y}, {xacc, yacc} ->
      {x + xacc, y + yacc}
    end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_directions("input.txt")
    |> HexTiler.load_initial_state
    |> HexTiler.count(:black)
  end

  def solve2() do
    load_directions("input.txt")
    |> HexTiler.load_initial_state
    |> HexTiler.run_for(100)
    |> HexTiler.count(:black)
  end

  defp load_directions(filename) do
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
