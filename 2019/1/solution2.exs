defmodule Fuel do
  def formula(x), do: div(x, 3) - 2

  def calculator(current_block, sum) when current_block <= 0, do: sum - current_block
  def calculator(current_block, sum) do
    new_addition = Fuel.formula(current_block)
    Fuel.calculator(new_addition, sum + new_addition)
  end
end

File.stream!("input.txt")
|> Enum.map(&String.trim/1)
|> Enum.map(&String.to_integer/1)
|> Enum.map(&Fuel.calculator(&1, 0))
|> Enum.sum
|> IO.inspect
