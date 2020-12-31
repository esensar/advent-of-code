defmodule WireGrid do
  @type point() :: {integer(), integer()}

  @spec generate_path(point(), String.t()) :: [point()]
  defp generate_path(start_point, instruction) do
    distance = String.slice(instruction, 1, String.length(instruction) - 1)
               |> String.trim
               |> String.to_integer
    case String.at(instruction, 0) do
      "R" -> ((elem(start_point, 0) + 1)..(elem(start_point, 0) + distance))
      |> Enum.map(fn x -> {x, elem(start_point, 1)} end)
      "L" -> ((elem(start_point, 0) - 1)..(elem(start_point, 0) - distance))
      |> Enum.map(fn x -> {x, elem(start_point, 1)} end)
      "U" -> ((elem(start_point, 1) + 1)..(elem(start_point, 1) + distance))
      |> Enum.map(fn x -> {elem(start_point, 0), x} end)
      "D" -> ((elem(start_point, 1) - 1)..(elem(start_point, 1) - distance))
      |> Enum.map(fn x -> {elem(start_point, 0), x} end)
    end
  end

  @spec generate_full_path([String.t()]) :: [point()]
  def generate_full_path(instruction_set) do
    generate_path_portion([{0, 0}], instruction_set, 0)
    |> Enum.split(1)
    |> Kernel.elem(1)
  end

  @spec intersections([point()], [point()]) :: [point()]
  def intersections(this_path, other_path) do
    MapSet.new(this_path)
    |> MapSet.intersection(MapSet.new(other_path))
    |> MapSet.to_list
  end

  @spec generate_path_portion([point()], [String.t()], integer()) :: [point()]
  defp generate_path_portion(current_path, instruction_set, current_instruction_index) do
    if current_instruction_index >= Enum.count(instruction_set) do
      current_path
    else
      new_path = current_path ++ generate_path(List.last(current_path), Enum.at(instruction_set, current_instruction_index))
      generate_path_portion(new_path, instruction_set, current_instruction_index + 1)
    end
  end
end

defmodule ProblemSolver do

  def solve1() do
    load_instruction_sets("input.txt")
    |> Enum.map(&WireGrid.generate_full_path/1)
    |> List.to_tuple
    |> get_intersections
    |> calculate_intersection_distances
    |> Enum.min
  end

  def solve2() do
    {first_path, second_path} = load_instruction_sets("input.txt")
                                |> Enum.map(&WireGrid.generate_full_path/1)
                                |> List.to_tuple
    intersections = get_intersections({first_path, second_path})
    second_path_steps = get_intersection_steps(second_path, intersections)
                        |> Enum.with_index
    get_intersection_steps(first_path, intersections)
    |> Enum.with_index
    |> Enum.map(fn {x, index} -> x + elem(Enum.at(second_path_steps, index), 0) end)
    |> Enum.min
  end

  @spec get_intersections({[WireGrid.point()], [WireGrid.point()]}) :: [WireGrid.point()]
  defp get_intersections({this_path, other_path}), do: WireGrid.intersections(this_path, other_path)

  @spec get_intersection_steps([WireGrid.point()], [WireGrid.point()]) :: [integer()]
  defp get_intersection_steps(path, intersections) do
    intersections
    |> Enum.map(fn x -> Enum.find_index(path, fn point -> point == x end) + 1 end)
  end

  @spec calculate_intersection_distances([WireGrid.point()]) :: [integer()]
  defp calculate_intersection_distances(intersections) do
    intersections
    |> Enum.map(fn x -> abs(elem(x, 0)) + abs(elem(x, 1)) end)
  end

  @spec generate_instruction_set(String.t()) :: [String.t()]
  defp generate_instruction_set(instruction_line) do
    instruction_line
    |> String.split(",")
    |> Enum.map(&String.trim/1)
  end

  @spec load_instruction_sets(String.t()) :: [[String.t()]]
  defp load_instruction_sets(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&generate_instruction_set/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
