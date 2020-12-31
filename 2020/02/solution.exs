defmodule Password do
  @enforce_keys [:first, :second, :char, :password]
  defstruct [:first, :second, :char, :password]

  defmodule SledsShop do
    defp valid?(password) do
      password.password
      |> String.graphemes
      |> Enum.count(&Kernel.==(&1, password.char))
      |> count_in_range?(password)
    end

    defp count_in_range?(count, password) do
      count >= password.first and count <= password.second
    end

    def count_valid(passwords) do
      passwords
      |> Enum.count(&valid?/1)
    end
  end

  defmodule TobogganCo do
    defp valid?(password) do
      password_chars = password.password
      |> String.graphemes

      matches = [password.first - 1, password.second - 1]
      |> Enum.count(fn x -> Enum.at(password_chars, x) == password.char end)

      matches == 1
    end

    def count_valid(passwords) do
      passwords
      |> Enum.count(&valid?/1)
    end
  end

  def parse_line(line) do
    { char_config, password } = String.split(line, ":")
                                |> Enum.map(&String.trim/1)
                                |> List.to_tuple
    { count_config, char } = String.split(char_config, " ")
                             |> List.to_tuple
    { first , second } = String.split(count_config, "-")
                    |> Enum.map(&String.to_integer/1)
                    |> List.to_tuple
    %Password{first: first, second: second, char: char, password: password}
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_entries("input.txt")
    |> Password.SledsShop.count_valid
  end

  def solve2() do
    load_entries("input.txt")
    |> Password.TobogganCo.count_valid
  end

  defp load_entries(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> Enum.map(&Password.parse_line/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
