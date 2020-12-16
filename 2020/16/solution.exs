defmodule TicketTranslator do
  def parse_tickets_info({rules, my_ticket, other_tickets}) do
    {parse_rules(rules), Enum.at(parse_tickets(my_ticket), 0), parse_tickets(other_tickets)}
  end

  def find_invalid_values(rules, tickets) do
    tickets
    |> Enum.flat_map(fn x -> x end)
    |> Enum.reject(fn x ->
      rules
      |> Enum.map(&valid_value?(&1, x))
      |> Enum.reduce(&Kernel.or/2)
    end)
  end

  def find_field_positions(rules, tickets) do
    tickets = tickets
    # Discard invalid tickets
    |> Enum.filter(fn x ->
      Enum.map(x, fn value ->
        rules
        |> Enum.map(&valid_value?(&1, value))
        |> Enum.reduce(&Kernel.or/2)
      end)
      |> Enum.reduce(&Kernel.and/2)
    end)

    rule_positions = rules
    # Find valid positions for each of the rules
    |> Enum.map(fn {rule_name, rule} ->
      indexes = Enum.map(tickets, fn ticket ->
        ticket
        |> Enum.with_index
        |> Enum.filter(fn {value, _} ->
          valid_value?({rule_name, rule}, value)
        end)
        |> Enum.map(&elem(&1, 1))
      end)
      |> Enum.map(&MapSet.new/1)
      |> Enum.reduce(fn x, acc ->
        MapSet.intersection(x, acc)
      end)
      {rule_name, indexes}
    end)

    rule_positions
    |> Enum.sort_by(fn {_, indexes} -> Enum.count(indexes) end)
    |> Enum.reduce([], fn {rule_name, indexes}, acc ->
      if Enum.count(acc) == 0 do
        [{rule_name, indexes}]
      else
        used_indexes = acc
                       |> Enum.map(&elem(&1, 1))
                       |> Enum.reduce(&MapSet.union/2)
        acc ++ [{rule_name, MapSet.difference(indexes, used_indexes)}]
      end
    end)
    |> Enum.map(fn {rule_name, indexes} -> {rule_name, Enum.at(indexes, 0)} end)
  end

  def map_out_ticket(field_positions, ticket) do
    field_positions
    |> Enum.map(fn {field, index} ->
      {field, Enum.at(ticket, index)}
    end)
  end

  defp valid_value?({_, rule}, value) do
    rule
    |> Tuple.to_list
    |> Enum.map(fn x -> 
      satisfies_range?(x, value)
    end)
    |> Enum.reduce(&Kernel.or/2)
  end

  defp satisfies_range?({lower_bound, upper_bound}, value) when value >= lower_bound and value <= upper_bound, do: true
  defp satisfies_range?(_, _), do: false

  defp parse_rules(rules) do
    rules
    |> String.trim
    |> String.split("\n")
    |> Enum.map(&String.trim/1)
    |> Enum.map(fn x -> 
      {name, value_rules} = List.to_tuple(String.split(x, ": "))
      {l_value_rule, r_value_rule} = String.split(value_rules, " or ")
                                     |> Enum.map(&String.split(&1, "-"))
                                     |> Enum.map(fn x ->
                                       x
                                       |> Enum.map(&String.trim/1)
                                       |> Enum.map(&String.to_integer/1)
                                     end)
                                     |> Enum.map(&List.to_tuple/1)
                                     |> List.to_tuple
      {name, {l_value_rule, r_value_rule}}
    end)
  end

  defp parse_tickets(tickets) do
    tickets
    |> String.trim
    |> String.split("\n")
    |> Kernel.tl
    |> Enum.map(&String.split(&1, ","))
    |> Enum.map(fn x -> 
      Enum.map(x, &String.to_integer/1)
    end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    {rules, _, other_tickets} = load_tickets("input.txt")

    TicketTranslator.find_invalid_values(rules, other_tickets)
    |> Enum.sum
  end

  def solve2() do
    {rules, my_ticket, other_tickets} = load_tickets("input.txt")

    TicketTranslator.find_field_positions(rules, other_tickets)
    |> TicketTranslator.map_out_ticket(my_ticket)
    |> Enum.filter(fn {field_name, _} -> String.starts_with?(field_name, "departure") end)
    |> Enum.map(&elem(&1, 1))
    |> Enum.reduce(&Kernel.*/2)
  end

  defp load_tickets(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.trim
    |> String.split("\n\n")
    |> List.to_tuple
    |> TicketTranslator.parse_tickets_info
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
