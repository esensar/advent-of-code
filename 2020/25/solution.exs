defmodule Mitm do
  @div_const 20201227

  def transform(subject \\ 7, loops_left \\ 1, current_val \\ 1)
  def transform(_, 0, current_val), do: current_val
  def transform(subject, loop_counter, current_val) do
    transform(subject, loop_counter - 1, rem(current_val * subject, @div_const))
  end

  def find_loopsize(key, subject \\ 7, current_val \\ 1, loop_counter \\ 0)
  def find_loopsize(key, _, key, loop_counter), do: loop_counter
  def find_loopsize(key, subject, current_val, loop_counter) do
    find_loopsize(key, subject, rem(current_val * subject, @div_const), loop_counter + 1)
  end
end

defmodule ProblemSolver do
  def solve1() do
    {key1, key2} = load_public_keys("input.txt")
    loop_size = Mitm.find_loopsize(key1)
    Mitm.transform(key2, loop_size)
  end

  def solve2() do
    "Merry Christmas"
  end

  defp load_public_keys(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
