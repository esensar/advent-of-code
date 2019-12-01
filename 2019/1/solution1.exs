File.stream!("input.txt")
|> Enum.map(&String.trim/1)
|> Enum.map(&String.to_integer/1)
|> Enum.map(fn x -> div(x, 3) - 2 end)
|> Enum.sum
|> IO.inspect
