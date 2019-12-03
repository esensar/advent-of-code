defmodule Intcode do
  @opcode_add 1
  @opcode_multiply 2
  @opcode_end 99

  @spec execute(list(integer)) :: list(integer)
  def execute(list) when is_list(list) do
    execute_program(list, 0)
  end
  def execute(list = [@opcode_end | _tail]), do: list
  def execute([]), do: []

  def get_result(list), do: Enum.at(list, 0)

  defp execute_program(list, current_index) do
    case run_operation(list, current_index) do
      {:continue, new_list} -> execute_program(new_list, current_index + 4)
      {:end, new_list} -> new_list
    end
  end

  defp run_operation(list, current_index) do
    execute_operation(Enum.at(list, current_index), list, current_index)
  end

  defp execute_operation(@opcode_add, list, current_index) do
    {:continue,
      list
      |> List.replace_at(
        Enum.at(list, current_index + 3),
        Enum.at(list, Enum.at(list, current_index + 1)) + Enum.at(list, Enum.at(list, current_index + 2))
      )}
  end
  defp execute_operation(@opcode_multiply, list, current_index) do
    {:continue, 
      list
      |> List.replace_at(
        Enum.at(list, current_index + 3),
        Enum.at(list, Enum.at(list, current_index + 1)) * Enum.at(list, Enum.at(list, current_index + 2))
      )}
  end
  defp execute_operation(@opcode_end, list, _current_index), do: {:end, list}
end

defmodule IntcodeLoader do
  def prepare_from_file(filename) do
    File.stream!(filename)
    |> Enum.flat_map(&String.split(&1, ","))
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.to_integer/1)
  end

  def setup_noun_and_verb(intcode, noun, verb) do
    intcode
    |> List.replace_at(1, noun)
    |> List.replace_at(2, verb)
  end
end

defmodule ProblemSolver do
  @problem2_expected_output 19690720

  def solve1() do
    IntcodeLoader.prepare_from_file("input.txt")
    |> IntcodeLoader.setup_noun_and_verb(12, 02)
    |> Intcode.execute
    |> Intcode.get_result
  end

  def solve2() do
    {:done, noun, verb} = check_nouns_and_verbs(0, 0)
    noun * 100 + verb
  end

  defp try_noun_and_verb(noun, verb) do
    result = IntcodeLoader.prepare_from_file("input.txt")
             |> IntcodeLoader.setup_noun_and_verb(noun, verb)
             |> Intcode.execute
             |> Intcode.get_result
    result == @problem2_expected_output
  end

  defp check_nouns_and_verbs(noun, verb) do
    if try_noun_and_verb(noun, verb) do
      {:done, noun, verb}
    else
      if verb >= 99 do
        check_nouns_and_verbs(noun + 1, 0)
      else
        check_nouns_and_verbs(noun, verb + 1)
      end
    end
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
