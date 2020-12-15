defmodule ElfMemoryGame do
  def find_number_at(starting_numbers, target_index) do
    game_state = starting_numbers
    |> Enum.with_index
    |> Map.new

    {_, last_number} = play_until(game_state, List.last(starting_numbers), Enum.count(starting_numbers) - 1, target_index - 1)
    last_number
  end

  defp play_until(game_state, last_number, last_index, target_index) when last_index == target_index, do: {game_state, last_number}
  defp play_until(game_state, last_number, last_index, target_index) do
    new_number = if Map.has_key?(game_state, last_number) do
      last_index - Map.get(game_state, last_number)
    else
      0
    end
    new_index = last_index + 1
    play_until(Map.put(game_state, last_number, last_index), new_number, new_index, target_index)
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_starting_numbers("input.txt")
    |> ElfMemoryGame.find_number_at(2020)
  end

  def solve2() do
    load_starting_numbers("input.txt")
    |> ElfMemoryGame.find_number_at(30000000)
  end

  defp load_starting_numbers(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> Enum.at(0)
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
