defmodule NavComputer do

  defmodule ShipState do
    @enforce_keys [:north, :east, :direction]
    defstruct [:north, :east, :direction]

    @directions %{N: 0, S: 180, E: 90, W: 270}

    def update(state, {:N, value}), do: %ShipState{state | north: state.north + value}
    def update(state, {:S, value}), do: %ShipState{state | north: state.north - value}
    def update(state, {:E, value}), do: %ShipState{state | east: state.east + value}
    def update(state, {:W, value}), do: %ShipState{state | east: state.east - value}
    def update(state, {:L, value}), do: %ShipState{state | direction: update_angle(state.direction, value, :L)}
    def update(state, {:R, value}), do: %ShipState{state | direction: update_angle(state.direction, value, :R)}
    def update(state, {:F, value}), do: update(state, {state.direction, value})

    defp update_angle(current_angle, angle_mod, direction) when is_atom(current_angle) do
      update_angle(Map.get(@directions, current_angle), angle_mod, direction)
    end
    defp update_angle(current_angle, angle_mod, :L) do
      angle_to_direction(current_angle - angle_mod)
    end
    defp update_angle(current_angle, angle_mod, :R) do
      angle_to_direction(current_angle + angle_mod)
    end

    defp angle_to_direction(angle) when angle >= 360, do: angle_to_direction(angle - 360)
    defp angle_to_direction(angle) when angle < 0, do: angle_to_direction(angle + 360)
    defp angle_to_direction(angle) do
      @directions
      |> Map.to_list
      |> Enum.map(fn {l, r} -> {r, l} end)
      |> Map.new
      |> Map.get(angle)
    end
  end

  defmodule ShipWithWaypointState do
    @enforce_keys [:north, :east, :waypoint_north, :waypoint_east]
    defstruct [:north, :east, :waypoint_north, :waypoint_east]

    def update(state, {:N, value}), do: %ShipWithWaypointState{state | waypoint_north: state.waypoint_north + value}
    def update(state, {:S, value}), do: %ShipWithWaypointState{state | waypoint_north: state.waypoint_north - value}
    def update(state, {:E, value}), do: %ShipWithWaypointState{state | waypoint_east: state.waypoint_east + value}
    def update(state, {:W, value}), do: %ShipWithWaypointState{state | waypoint_east: state.waypoint_east - value}
    def update(state, {:L, value}), do: rotate_waypoint(state, -value)
    def update(state, {:R, value}), do: rotate_waypoint(state, value)
    def update(state, {:F, value}), do: %ShipWithWaypointState{state | north: state.north + value * state.waypoint_north, east: state.east + value * state.waypoint_east}

    defp rotate_waypoint(state, value) do
      value = normalize_angle(value)
      case value do
        0 -> state
        90 -> %ShipWithWaypointState{state | waypoint_north: -state.waypoint_east, waypoint_east: state.waypoint_north}
        180 -> %ShipWithWaypointState{state | waypoint_north: -state.waypoint_north, waypoint_east: -state.waypoint_east}
        270 -> %ShipWithWaypointState{state | waypoint_north: state.waypoint_east, waypoint_east: -state.waypoint_north}
      end
    end

    defp normalize_angle(angle) when angle >= 360, do: normalize_angle(angle - 360)
    defp normalize_angle(angle) when angle < 0, do: normalize_angle(angle + 360)
    defp normalize_angle(angle), do: angle
  end

  def run_instructions(instructions, starting_state) do
    instructions
    |> Enum.reduce(starting_state, fn x, acc -> run_instruction(x, acc) end)
  end

  def run_instructions_with_waypoint(instructions, starting_state) do
    instructions
    |> Enum.reduce(starting_state, fn x, acc -> run_instruction_with_waypoint(x, acc) end)
  end

  defp run_instruction_with_waypoint(instruction, state) do
    ShipWithWaypointState.update(state, instruction)
  end

  defp run_instruction(instruction, state) do
    ShipState.update(state, instruction)
  end

  def parse_instructions(lines) do
    lines
    |> Enum.map(&String.graphemes/1)
    |> Enum.map(fn [instruction | value] ->
      {String.to_atom(instruction), Enum.join(value, "") |> String.to_integer}
    end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    final_state = load_instructions("input.txt")
    |> NavComputer.run_instructions(%NavComputer.ShipState{north: 0, east: 0, direction: :E})

    abs(final_state.north) + abs(final_state.east)
  end

  def solve2() do
    final_state = load_instructions("input.txt")
    |> NavComputer.run_instructions_with_waypoint(%NavComputer.ShipWithWaypointState{north: 0, east: 0, waypoint_north: 1, waypoint_east: 10})

    abs(final_state.north) + abs(final_state.east)
  end

  defp load_instructions(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> NavComputer.parse_instructions
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
