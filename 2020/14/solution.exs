defmodule DockerAdapter do
  @default_mask "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"

  def execute(instructions, memory \\ %{}, mask \\ @default_mask)
  def execute([], memory, _), do: memory
  def execute(instructions, memory, mask) do
    case Enum.at(instructions, 0) do
      {:mask, value} -> execute(Enum.drop(instructions, 1), memory, value)
      {:mem, args} -> update_state_and_execute(Enum.drop(instructions, 1), memory, mask, args)
    end
  end

  defp update_state_and_execute(instructions, memory, mask, {address, value}) do
    masked_value = mask_value(mask, value)
    memory = Map.put(memory, address, masked_value)
    execute(instructions, memory, mask)
  end

  defp mask_value(mask, value) do
    binary = Integer.to_string(value, 2)
    |> String.graphemes
    |> Enum.reverse

    mask
    |> String.graphemes
    |> Enum.reverse
    |> Enum.with_index
    |> Enum.map(fn {x, index} ->
      case x do
        "X" -> if index >= Enum.count(binary), do: "0", else: Enum.at(binary, index)
        _ -> x
      end
    end)
    |> Enum.reverse
    |> Enum.join("")
    |> String.to_integer(2)
  end

  defmodule V2 do
    @default_mask "000000000000000000000000000000000000"

    def execute(instructions, memory \\ %{}, mask \\ @default_mask)
    def execute([], memory, _), do: memory
    def execute(instructions, memory, mask) do
      case Enum.at(instructions, 0) do
        {:mask, value} -> execute(Enum.drop(instructions, 1), memory, value)
        {:mem, args} -> update_state_and_execute(Enum.drop(instructions, 1), memory, mask, args)
      end
    end

    defp update_state_and_execute(instructions, memory, mask, {address, value}) do
      memory = mask_address(mask, address)
               |> Enum.reduce(memory, fn x, mem ->
                 Map.put(mem, x, value)
               end)
      execute(instructions, memory, mask)
    end

    defp mask_address(mask, address) do
      binary = Integer.to_string(address, 2)
      |> String.graphemes
      |> Enum.reverse

      mask
      |> String.graphemes
      |> Enum.reverse
      |> Enum.with_index
      |> Enum.map(fn {x, index} ->
        case x do
          "0" -> if index >= Enum.count(binary), do: "0", else: Enum.at(binary, index)
          "1" -> "1"
          "X" -> "X"
        end
      end)
      |> Enum.reverse
      |> Enum.reduce([[]], fn x, acc ->
        case x do
          "X" -> (
              Enum.map(acc, fn address -> address ++ ["0"] end) ++
              Enum.map(acc, fn address -> address ++ ["1"] end))
          val -> Enum.map(acc, fn address -> address ++ [val] end)
        end
      end)
      |> Enum.map(&Enum.join(&1, ""))
      |> Enum.map(&String.to_integer(&1, 2))
    end
  end

  def parse_instructions(lines) do
    lines
    |> Enum.map(&String.split(&1, " = "))
    |> Enum.map(&List.to_tuple/1)
    |> Enum.map(fn {instruction, value} ->
      case Regex.scan(~r/(mask|mem)\[?(\d*)?\]?/, instruction) do
        [[_, "mask", _]] ->
          {:mask, value}
        [[_, "mem", address]] ->
          {:mem, {String.to_integer(address), String.to_integer(value)}}
      end
    end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_instructions("input.txt")
    |> DockerAdapter.execute
    |> Map.values
    |> Enum.sum
  end

  def solve2() do
    load_instructions("input.txt")
    |> DockerAdapter.V2.execute
    |> Map.values
    |> Enum.sum
  end

  defp load_instructions(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> DockerAdapter.parse_instructions
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
