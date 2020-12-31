defmodule LoopDetector do

  def execute(instructions, current_line \\ 0, acc \\ 0, executed_instructions \\ []) do
    {next_line, acc} = run_instruction(current_line, acc, Enum.at(instructions, current_line))

    cond do
      next_line in executed_instructions -> {:fail, acc}
      next_line >= Enum.count(instructions) -> {:ok, acc}
      true -> execute(instructions, next_line, acc, executed_instructions ++ [current_line])
    end
  end

  def find_working_acc_value(instructions, start_at \\ 0) do
    index = instructions
            |> Enum.slice(start_at..(Enum.count(instructions) - 1))
            |> Enum.find_index(fn {op, _} -> op == :jmp or op == :nop end)
    index = index + start_at
    replacement = case instructions
                  |> Enum.at(index) do
      {:jmp, number} -> {:nop, number}
      {:nop, number} -> {:jmp, number}
    end
    modified = instructions
               |> List.replace_at(index, replacement)
    case execute(modified) do
      {:ok, acc} -> acc
      {:fail, acc} -> find_working_acc_value(instructions, index + 1)
    end
  end

  def get_acc_before_looping(instructions) do
    {:fail, acc} = execute(instructions)
    acc
  end

  defp run_instruction(line, acc, {:nop, _}), do: {line + 1, acc}
  defp run_instruction(line, acc, {:jmp, arg}), do: {line + arg, acc}
  defp run_instruction(line, acc, {:acc, arg}), do: {line + 1, acc + arg}

  def parse_instruction(instruction) do
    {type, arg} = instruction
                  |> String.split(" ")
                  |> List.to_tuple

    arg = case Regex.scan(~r/([+-])(\d*)/, arg) do
      [[_, "+", number]] -> String.to_integer(number)
      [[_, "-", number]] -> -String.to_integer(number)
      _ -> raise "bad instruction #{instruction}"
    end

    {String.to_atom(type), arg}
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_instructions("input.txt")
    |> LoopDetector.get_acc_before_looping
  end

  def solve2() do
    load_instructions("input.txt")
    |> LoopDetector.find_working_acc_value
  end

  defp load_instructions(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> Enum.map(&LoopDetector.parse_instruction/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
