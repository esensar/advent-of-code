defmodule XMasBreaker do

  defp valid?(preamble, number) do
    count = preamble
    |> Enum.map(&Kernel.-(number, &1))
    |> Enum.count(&Kernel.in(&1, preamble))

    count > 0
  end

  defp find_weak_set(numbers, target, curr \\ 0) do
    potential_set = numbers
    |> Enum.with_index
    |> Enum.filter(fn {_, index} -> index >= curr end)
    |> Enum.map(&elem(&1, 0))

    {value, weak_set} = potential_set
    |> Enum.reduce_while({0, []}, fn x, {sum, items} = acc ->
      if (x + sum) > target, do: {:halt, acc}, else: {:cont, {sum + x, items ++ [x]}}
    end)

    if value == target, do: weak_set, else: find_weak_set(numbers, target, curr + 1)
  end

  def first_invalid(numbers, preamble_size) do
    numbers
    |> Enum.with_index
    |> Enum.filter(fn {_, index} -> index >= preamble_size end)
    |> Enum.find(fn {x, index} -> not valid?(Enum.slice(numbers, index - preamble_size, preamble_size), x) end)
    |> elem(0)
  end

  def find_weakness(numbers, preamble_size) do
    invalid_number = first_invalid(numbers, preamble_size)

    weak_set = find_weak_set(numbers, invalid_number)

    Enum.max(weak_set) + Enum.min(weak_set)
  end

end

defmodule ProblemSolver do
  def solve1() do
    load_numbers("input.txt")
    |> XMasBreaker.first_invalid(25)
  end

  def solve2() do
    load_numbers("input.txt")
    |> XMasBreaker.find_weakness(25)
  end

  defp load_numbers(filename) do
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
