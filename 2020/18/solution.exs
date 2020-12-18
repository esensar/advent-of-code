defmodule BasicTokenizer do
  def tokenize_expression(expression) do
    expression
    |> String.replace(" ", "")
    |> String.graphemes
    |> Enum.chunk_while(
      "",
      fn x, acc ->
        if acc == "" do
          {:cont, acc <> x}
        else
          first = String.first(acc)
          cond do
            first == "(" ->
              if (Enum.count(String.graphemes(acc), fn x -> x == "(" end) - Enum.count(String.graphemes(acc), fn x -> x == ")" end)) > 1 do
                  {:cont, acc <> x}
                else
                if x != ")" do
                  {:cont, acc <> x}
                else
                  {:cont, acc <> x, ""}
                end
                end
            first in ["*", "+"] -> {:cont, acc, x}
            true ->
              cond do
                x in ["*", "+"] -> {:cont, acc, x}
                x in ["(", ")"] -> {:cont, acc, x}
                true -> {:cont, acc <> x}
              end
          end
        end
      end,
        fn 
          "" -> {:cont, ""}
          acc -> {:cont, acc, ""}
        end
    )
  end
end

defmodule DumbMathEngine do
  def calculate_expression(expression) do
    BasicTokenizer.tokenize_expression(expression)
    |> Enum.reduce({"+", 0}, fn x, {last_op, value} ->
      cond do
        x in ["*", "+"] -> {x, value}
        last_op == "+" -> {last_op, value + calculate_subexpression(x)}
        last_op == "*" -> {last_op, value * calculate_subexpression(x)}
      end
    end)
    |> elem(1)
  end

  defp calculate_subexpression(subexpression) do
    case String.first(subexpression) do
      "(" -> calculate_expression(String.slice(subexpression, 1..(String.length(subexpression)-2)))
      _ -> String.to_integer(String.trim(subexpression))
    end
  end
end

defmodule WeirdMathEngine do
  def calculate_expression(expression) do
    BasicTokenizer.tokenize_expression(expression)
    # Do + first
    |> Enum.reduce([], fn x, acc ->
      if acc == [] do
        [x]
      else
        last_token = List.last(acc)
        if last_token == "+" do
          Enum.take(acc, Enum.count(acc) - 2) ++ [Integer.to_string(calculate_subexpression(Enum.at(acc, -2)) + calculate_subexpression(x))]
        else
          acc ++ [x]
        end
      end
    end)
    |> Enum.reduce({"+", 0}, fn x, {last_op, value} ->
      cond do
        x in ["*", "+"] -> {x, value}
        last_op == "+" -> {last_op, value + calculate_subexpression(x)}
        last_op == "*" -> {last_op, value * calculate_subexpression(x)}
      end
    end)
    |> elem(1)
  end

  defp calculate_subexpression(subexpression) do
    case String.first(subexpression) do
      "(" -> calculate_expression(String.slice(subexpression, 1..(String.length(subexpression)-2)))
      _ -> String.to_integer(String.trim(subexpression))
    end
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_expressions("input.txt")
    |> Enum.map(&DumbMathEngine.calculate_expression/1)
    |> Enum.sum
  end

  def solve2() do
    load_expressions("input.txt")
    |> Enum.map(&WeirdMathEngine.calculate_expression/1)
    |> Enum.sum
  end

  defp load_expressions(filename) do
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
