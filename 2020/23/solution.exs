defmodule CrapCups do
  def play(cups, moves, current_cup, max_val)
  def play(cups, 0, _, _), do: cups
  def play(cups, moves, current_cup, max_val) do
    {taken_cups, new_end, last_taken_cup} = take_cups(cups, Map.get(cups, current_cup))
    new_cups = Map.drop(cups, Map.keys(taken_cups))
               |> Map.put(current_cup, new_end)
               |> put_cups(current_cup - 1, Map.get(cups, current_cup), last_taken_cup, taken_cups, max_val)

    play(new_cups, moves - 1, Map.get(new_cups, current_cup), max_val)
  end

  defp take_cups(cups, current_cup, counter \\ 0, taken \\ Map.new, last_cup \\ 0) do
    if counter == 3 do
      {taken, current_cup, last_cup}
    else
      next_cup = Map.get(cups, current_cup)
      take_cups(cups, next_cup, counter + 1, Map.put(taken, current_cup, next_cup), current_cup)
    end
  end

  defp put_cups(cups, target_cup, cups_start, cups_end, taken_cups, max_val) do
    if Map.has_key?(cups, target_cup) do
      previous_target = Map.get(cups, target_cup)
      Map.put(cups, target_cup, cups_start)
      |> Map.merge(taken_cups)
      |> Map.put(cups_end, previous_target)
    else
      if target_cup <= 0 do
        put_cups(cups, max_val, cups_start, cups_end, taken_cups, max_val)
      else
        put_cups(cups, target_cup - 1, cups_start, cups_end, taken_cups, max_val)
      end
    end
  end

  def prepare_cups(cups) do
    cups_size = Enum.count(cups)
    cups
    |> Enum.with_index
    |> Enum.reduce(Map.new, fn {x, index}, acc ->
      Map.put(acc, x, Enum.at(cups, rem(index + 1, cups_size)))
    end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    initial_cups = load_cups("input.txt")
    starting_cup = List.first(initial_cups)
    final_cups = load_cups("input.txt")
                 |> CrapCups.prepare_cups
                 |> CrapCups.play(100, starting_cup, Enum.max(initial_cups))

    print_cups(final_cups, Map.get(final_cups, 1))
  end

  defp print_cups(map, cup, result \\ "")
  defp print_cups(map, 1, result), do: result
  defp print_cups(map, cup, result) do
    print_cups(map, Map.get(map, cup), result <> Integer.to_string(cup))
  end

  def solve2() do
    initial_cups = load_cups("input.txt")
    starting_cup = List.first(initial_cups)
    last_cup = List.last(initial_cups)
    max = Enum.max(initial_cups)
    additional_cups =
      (max + 1)..999_999
      |> Enum.map(fn x -> {x, x + 1} end)
      |> Map.new
    final_cups = initial_cups
                 |> CrapCups.prepare_cups
                 |> Map.put(last_cup, max + 1)
                 |> Map.merge(additional_cups)
                 |> Map.put(1_000_000, starting_cup)
                 |> CrapCups.play(10_000_000, starting_cup, 1_000_000)


    next = Map.get(final_cups, 1)
    next * Map.get(final_cups, next)
  end

  defp load_cups(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.graphemes
    |> Enum.map(&String.trim/1)
    |> Enum.filter(fn x -> String.length(x) > 0 end)
    |> Enum.map(&String.to_integer/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
