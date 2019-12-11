defmodule OrbitMap do
  @center_of_mass "COM"

  @spec count_total_orbits_in_map(%{}) :: Integer.t()
  def count_total_orbits_in_map(orbit_map) do
    orbit_map
    |> Map.keys
    |> Enum.map(&count_orbits_to_center(orbit_map, &1))
    |> Enum.sum
  end

  @spec orbits_between_satellites(%{}, String.t(), String.t()) :: [String.t()]
  def orbits_between_satellites(orbit_map, first, second) do
    route_for_first = orbits_to_center(orbit_map, first)
                      |> Enum.with_index
    route_for_second = orbits_to_center(orbit_map, second)
                       |> Enum.with_index
    first_common_index = 
      Enum.find_value(
        route_for_first,
        @center_of_mass,
        fn x -> 
          Enum.find_value(
            route_for_second,
            fn y -> if elem(y, 0) == elem(x, 0), do: {elem(x, 1), elem(y, 1)}, else: false end
          ) 
        end
      )
    (elem(
      Enum.split(route_for_first, elem(first_common_index, 0)),
      0) ++ Enum.reverse(
        elem(Enum.split(route_for_second, elem(first_common_index, 1)),
          0))) 
          |> Enum.map(&Kernel.elem(&1, 0))
  end

  defp orbits_to_center(orbit_map, satellite, current_orbits \\ []) do
    if satellite == @center_of_mass do
      current_orbits
    else
      center = Map.get(orbit_map, satellite, @center_of_mass)
      orbits_to_center(orbit_map, center, current_orbits ++ [center])
    end
  end

  defp count_orbits_to_center(orbit_map, satellite) do
    orbits_to_center(orbit_map, satellite)
    |> Enum.count
  end
end

defmodule ProblemSolver do

  def solve1() do
    load_input("input.txt")
    |> OrbitMap.count_total_orbits_in_map
  end

  def solve2() do
    load_input("input.txt")
    |> OrbitMap.orbits_between_satellites("YOU", "SAN")
    |> IO.inspect
    |> Enum.count
  end

  @spec load_input(String.t()) :: %{}
  defp load_input(filename) do
    File.stream!(filename)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split(&1, ")"))
    |> Enum.map(&Enum.reverse/1)
    |> Enum.map(&List.to_tuple/1)
    |> Enum.into(%{})
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
