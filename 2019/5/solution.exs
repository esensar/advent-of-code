defmodule Intcode do
  @opcode_add 1
  @opcode_multiply 2
  @opcode_read 3
  @opcode_print 4
  @opcode_jmpif 5
  @opcode_jmpif_not 6
  @opcode_lt 7
  @opcode_eq 8
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
      {:continue, new_list, offset} -> execute_program(new_list, current_index + offset)
      {:end, new_list} -> new_list
    end
  end

  defp run_operation(list, current_index) do
    execute_operation(Enum.at(list, current_index), list, current_index)
  end

  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_add) do
    [first_param, second_param] = fetch_params(opcode, list, current_index, 2)
    {:continue,
      list
      |> List.replace_at(
        Enum.at(list, current_index + 3),
        first_param + second_param
      ), 4}
  end
  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_multiply) do
    [first_param, second_param] = fetch_params(opcode, list, current_index, 2)
    {:continue, 
      list
      |> List.replace_at(
        Enum.at(list, current_index + 3),
        first_param * second_param
      ), 4}
  end
  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_read) do
    {:continue,
      list
      |> List.replace_at(
        Enum.at(list, current_index + 1),
        IO.gets("") |> String.trim |> String.to_integer
      ), 2
    }
  end
  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_print) do
    [first_param] = fetch_params(opcode, list, current_index, 1)
    IO.puts(first_param)
    {:continue, list, 2}
  end
  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_jmpif) do
    [first_param, second_param] = fetch_params(opcode, list, current_index, 2)
    {:continue,
      list,
      if (first_param != 0) do
        second_param - current_index
      else
        3
      end
    }
  end
  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_jmpif_not) do
    [first_param, second_param] = fetch_params(opcode, list, current_index, 2)
    {:continue,
      list,
      if (first_param == 0) do
        second_param - current_index
      else
        3
      end
    }
  end
  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_eq) do
    [first_param, second_param] = fetch_params(opcode, list, current_index, 2)
    {:continue,
      list
      |> List.replace_at(
        Enum.at(list, current_index + 3),
        if (first_param == second_param) do
          1
        else
          0
        end
      ),
      4
    }
  end
  defp execute_operation(opcode, list, current_index) when (rem(opcode, 100) == @opcode_lt) do
    [first_param, second_param] = fetch_params(opcode, list, current_index, 2)
    {:continue,
      list
      |> List.replace_at(
        Enum.at(list, current_index + 3),
        if (first_param < second_param) do
          1
        else
          0
        end
      ),
      4
    }
  end
  defp execute_operation(@opcode_end, list, _current_index), do: {:end, list}

  defp fetch_params(opcode, list, current_index, param_count) do
    fetch_single_param(opcode, list, current_index, param_count, 1)
  end

  defp fetch_single_param(opcode, list, current_index, param_count, current_param) do
    if param_count < current_param do
      []
    else
      [(case (div(rem(opcode, :math.pow(10, current_param + 2) |> trunc), :math.pow(10, current_param + 1) |> trunc)) do
        1 -> Enum.at(list, current_index + current_param)
        0 -> Enum.at(list, Enum.at(list, current_index + current_param))
      end)] ++ fetch_single_param(opcode, list, current_index, param_count, current_param + 1)
    end
  end
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
  def solve1() do
    IntcodeLoader.prepare_from_file("input.txt")
    |> Intcode.execute
    |> Intcode.get_result
  end

  def solve2() do
    IntcodeLoader.prepare_from_file("input.txt")
    |> Intcode.execute
    |> Intcode.get_result
  end
end

IO.puts("Problem 1 solution: ")
IO.puts("Please input 1 when asked...")
ProblemSolver.solve1()

IO.puts("Problem 2 solution: ")
IO.puts("Please input 5 when asked...")
ProblemSolver.solve2()
