defmodule CubeMatrix do
  def run_boot_process(matrix, max_cycles, completed_cycles \\ 0)
  def run_boot_process(matrix, max_cycles, completed_cycles) when completed_cycles == max_cycles, do: matrix
  def run_boot_process(matrix, max_cycles, completed_cycles) do
    # Pad the matrix first
    first_layer = Enum.at(matrix, 0)
    new_layer = -1..Enum.count(first_layer)
                |> Enum.map(fn _ ->
                  -1..Enum.count(Enum.at(first_layer, 0))
                  |> Enum.map(fn _ ->
                    "."
                  end)
                end)
    matrix = matrix
             |> Enum.map(fn layer ->
               new_col = -1..Enum.count(Enum.at(layer, 0))
                           |> Enum.map(fn _ ->
                             "."
                           end)
               new_layer = layer
                           |> Enum.map(fn row ->
                             ["."] ++ row ++ ["."]
                           end)
               [new_col] ++ new_layer ++ [new_col]
             end)
    matrix = [new_layer] ++ matrix ++ [new_layer]

    # Run the rule on each cube
    new_matrix = matrix
                 |> Enum.with_index
                 |> Enum.map(fn {layer, layernum} ->
                   layer
                   |> Enum.with_index
                   |> Enum.map(fn {row, rownum} ->
                     row 
                     |> Enum.with_index
                     |> Enum.map(fn {_, colnum} ->
                       apply_rule(matrix, layernum, rownum, colnum)
                     end)
                   end)
                 end)

    run_boot_process(new_matrix, max_cycles, completed_cycles + 1)
  end

  defp apply_rule(matrix, layer, row, col) do
    cube = matrix
           |> Enum.at(layer) 
           |> Enum.at(row)
           |> Enum.at(col)

    case cube do
      "#" -> if count_adjacent_active(matrix, layer, row, col) in [2, 3], do: "#", else: "."
      "." -> if count_adjacent_active(matrix, layer, row, col) == 3, do: "#", else: "."
    end
  end

  defp count_adjacent_active(matrix, layer, row, col) do
  -1..1
  |> Enum.flat_map(fn l -> 
  -1..1
  |> Enum.flat_map(fn r -> 
    -1..1
    |> Enum.map(fn c -> 
      {l, r, c} 
    end)
  end)
  end)
  |> Enum.filter(fn {l, r, c} -> l != 0 || r != 0 || c != 0 end)
  |> Enum.count(fn {l, r, c} -> active?(matrix, layer + l, row + r, col + c) end)
  end

  defp active?(matrix, layer, row, col) do
    cond do
      layer < 0 or layer >= Enum.count(matrix) -> false
      row < 0 or row >= Enum.count(Enum.at(matrix, 0)) -> false
      col < 0 or col >= Enum.count(Enum.at(Enum.at(matrix, 0), 0)) -> false
      true -> (matrix
      |> Enum.at(layer) 
      |> Enum.at(row)
      |> Enum.at(col)) == "#"
    end
  end
end

# Laziness
# Just copied over and modified
defmodule CubeMatrix4d do
  def run_boot_process(matrix, max_cycles, completed_cycles \\ 0)
  def run_boot_process(matrix, max_cycles, completed_cycles) when completed_cycles == max_cycles, do: matrix
  def run_boot_process(matrix, max_cycles, completed_cycles) do
    # Pad the matrix first
    first_hyper = Enum.at(matrix, 0)
    new_hyper = -1..Enum.count(first_hyper)
                |> Enum.map(fn _ ->
                  -1..Enum.count(Enum.at(first_hyper, 0))
                  |> Enum.map(fn _ ->
                    -1..Enum.count(Enum.at(Enum.at(first_hyper, 0), 0))
                    |> Enum.map(fn _ ->
                      "."
                    end)
                  end)
                end)
    matrix = matrix
             |> Enum.map(fn hyper ->
               first_layer = Enum.at(hyper, 0)
               new_layer = -1..Enum.count(first_layer)
                           |> Enum.map(fn _ ->
                           -1..Enum.count(Enum.at(first_layer, 0))
                           |> Enum.map(fn _ ->
                             "."
                           end)
                           end)
               new_hyper = hyper
                           |> Enum.map(fn layer ->
                             new_col = -1..Enum.count(Enum.at(layer, 0))
                                       |> Enum.map(fn _ ->
                                         "."
                                       end)
                             new_layer = layer
                                         |> Enum.map(fn row ->
                                           ["."] ++ row ++ ["."]
                                         end)
                             [new_col] ++ new_layer ++ [new_col]
                           end)
               [new_layer] ++ new_hyper ++ [new_layer]
             end)
    matrix = [new_hyper] ++ matrix ++ [new_hyper]

    # Run the rule on each cube
    new_matrix = matrix
                 |> Enum.with_index
                 |> Enum.map(fn {hyper, hypernum} ->
                   hyper
                   |> Enum.with_index
                   |> Enum.map(fn {layer, layernum} ->
                     layer
                     |> Enum.with_index
                     |> Enum.map(fn {row, rownum} ->
                       row 
                       |> Enum.with_index
                       |> Enum.map(fn {_, colnum} ->
                         apply_rule(matrix, hypernum, layernum, rownum, colnum)
                       end)
                     end)
                   end)
                 end)

    run_boot_process(new_matrix, max_cycles, completed_cycles + 1)
  end

  defp apply_rule(matrix, hyper, layer, row, col) do
    cube = matrix
           |> Enum.at(hyper) 
           |> Enum.at(layer) 
           |> Enum.at(row)
           |> Enum.at(col)

    case cube do
      "#" -> if count_adjacent_active(matrix, hyper, layer, row, col) in [2, 3], do: "#", else: "."
      "." -> if count_adjacent_active(matrix, hyper, layer, row, col) == 3, do: "#", else: "."
    end
  end

  defp count_adjacent_active(matrix, hyper, layer, row, col) do
  -1..1
  |> Enum.flat_map(fn h -> 
  -1..1
  |> Enum.flat_map(fn l -> 
    -1..1
    |> Enum.flat_map(fn r -> 
      -1..1
      |> Enum.map(fn c -> 
        {h, l, r, c} 
      end)
    end)
  end)
  end)
  |> Enum.filter(fn {h, l, r, c} -> h != 0 || l != 0 || r != 0 || c != 0 end)
  |> Enum.count(fn {h, l, r, c} -> active?(matrix, hyper + h, layer + l, row + r, col + c) end)
  end

  defp active?(matrix, hyper, layer, row, col) do
    cond do
      hyper < 0 or hyper >= Enum.count(matrix) -> false
      layer < 0 or layer >= Enum.count(Enum.at(matrix, 0)) -> false
      row < 0 or row >= Enum.count(Enum.at(Enum.at(matrix, 0), 0)) -> false
      col < 0 or col >= Enum.count(Enum.at(Enum.at(Enum.at(matrix, 0), 0), 0)) -> false
      true -> (matrix
      |> Enum.at(hyper) 
      |> Enum.at(layer) 
      |> Enum.at(row)
      |> Enum.at(col)) == "#"
    end
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_initial_state("input.txt")
    |> CubeMatrix.run_boot_process(6)
    |> Enum.flat_map(fn layer ->
      Enum.flat_map(layer, fn row -> row end)
    end)
    |> Enum.filter(&Kernel.==(&1, "#"))
    |> Enum.count
  end

  def solve2() do
    initial_4d = [load_initial_state("input.txt")]

    initial_4d
    |> CubeMatrix4d.run_boot_process(6)
    |> Enum.flat_map(fn hyper ->
      Enum.flat_map(hyper, fn layer ->
        Enum.flat_map(layer, fn row -> row end)
      end)
    end)
    |> Enum.filter(&Kernel.==(&1, "#"))
    |> Enum.count
  end

  defp load_initial_state(filename) do
    initial_layer = File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.graphemes/1)

    [initial_layer]
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
