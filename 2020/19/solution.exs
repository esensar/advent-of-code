defmodule MessageValidator do
  defp generate_rule_map(rules) do
    rules
    |> Enum.map(fn rule_line ->
      {rule_id, rule_rules} = String.split(rule_line, ": ")
                              |> List.to_tuple

      rule_rules = rule_rules
                   |> String.split("|")
                   |> Enum.map(fn rule_options ->
                     rule_options
                     |> String.split(" ")
                     |> Enum.filter(fn x -> String.length(x) > 0 end)
                   end)

      {rule_id, rule_rules}
    end)
    |> Map.new
  end

  def generate_valid_messages(rule_map, starting_rule, cache_key, max_depth) do
    :ets.new(cache_key, [:named_table])
    generate_valid_messages_for_rule(rule_map, starting_rule, cache_key, 0, max_depth)
  end

  defp generate_valid_messages_for_rule(rule_map, current_rule_id, cache_key, depth, max_depth) do
    if depth > max_depth do
      ["42 31"] # HACK: Stop recursion
    else
      cached = :ets.lookup(cache_key, current_rule_id)
      if Enum.count(cached) > 0 do
        {_, new_messages} = Enum.at(cached, 0)
        new_messages
      else
        current_rule = rule_map[current_rule_id]

        new_messages = current_rule
                       |> Enum.map(fn rule ->
                         # Assume that strings never come in | statements
                         if String.starts_with?(Enum.at(rule, 0), "\"") do
                           [String.trim(Enum.at(rule, 0), "\"")]
                         else
                           Enum.reduce(rule, [""], fn rule_item, acc ->
                             Enum.flat_map(generate_valid_messages_for_rule(rule_map, rule_item, cache_key, depth + 1, max_depth), fn message ->
                               Enum.map(acc, fn acc_message -> acc_message <> message end)
                             end)
                           end)
                         end
                       end)
                       |> Enum.flat_map(fn valid_message_group -> valid_message_group end)

        :ets.insert(cache_key, {current_rule_id, MapSet.new(new_messages)})
        MapSet.new(new_messages)
      end
    end
  end

  def count_valid({rules, messages}, starting_rule) do
    rule_map = generate_rule_map(rules)
    valid_messages = generate_valid_messages(rule_map, starting_rule, :message_cache, 102312093812)
    Enum.count(messages, fn x -> x in valid_messages end)
  end

  def count_valid_with_modified_rules({rules, messages}, starting_rule, modifications) do
    rule_map = modifications
               |> Enum.reduce(generate_rule_map(rules), fn {rule_id, rule}, acc ->
                 Map.put(acc, rule_id, rule)
               end)
               |> IO.inspect
    valid_messages = generate_valid_messages(rule_map, starting_rule, :modified_message_cache, 12)
    Enum.count(messages, fn x -> x in valid_messages end)
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_rules_and_messages("input.txt")
    |> MessageValidator.count_valid("0")
  end

  # UNSOLVED
  def solve2() do
    modifications = %{
      "8" => [["42"], ["42", "8"]],
      "11" => [["42", "31"], ["42", "11", "31"]],
    }

    load_rules_and_messages("input.txt")
    |> MessageValidator.count_valid_with_modified_rules("0", modifications)
  end

  defp load_rules_and_messages(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.trim
    |> String.split("\n\n")
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split(&1, "\n"))
    |> List.to_tuple
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
