defmodule BagRules do

  def parse_rule(rule) do
    {container, contents} = String.split(rule, "contain") 
                            |> Enum.map(&String.trim/1)
                            |> List.to_tuple


    {parse_container_name(container), parse_contents(contents)}
  end

  def find_holder_count_of(rules, bag) do
    find_holders_of(rules, bag)
    |> Enum.filter(fn x -> x != bag end)
    |> Enum.count
  end
  
  def find_contents_count_of(rules, bag) do
    rules
    |> Enum.find(&rule_for_bag?(&1, bag))
    |> Kernel.elem(1)
    |> Enum.map(&contents_formula(rules, &1))
    |> Enum.sum
  end

  defp contents_formula(rules, {}), do: 0
  defp contents_formula(rules, {container, count}) do
    count + count * find_contents_count_of(rules, container)
  end

  defp find_holders_of(rules, bag) do
    rules
    |> Enum.filter(&holds_bag?(&1, bag))
    |> Enum.map(&Kernel.elem(&1, 0))
    |> Enum.flat_map(&find_holders_of(rules, &1))
    |> MapSet.new
    |> MapSet.put(bag)
  end

  defp rule_for_bag?({}, _), do: false
  defp rule_for_bag?({contents, _}, bag) when contents == bag, do: true
  defp rule_for_bag?({_, _}, _), do: false

  defp holds_bag?({}, _), do: false
  defp holds_bag?({_, contents}, bag) do
    count = contents
            |> Enum.count(
              fn entry -> 
                case entry do
                  {} -> false
                  {contained_bag, _} -> contained_bag == bag
                end
              end
            )

    count > 0
  end

  defp parse_contents(contents) do
    contents
    |> String.split(", ")
    |> Enum.map(&parse_contents_entry/1)
  end

  defp parse_contents_entry(entry) do
    case Regex.scan(~r/^(\d*|no)\ ([\w\ ]*)\.?$/, entry) do
      [[_, "no", _]] -> {}
      [[_, count, bag]] -> {parse_container_name(bag), String.to_integer(count)}
      _ -> raise "bad entry #{entry}"
    end
  end

  defp parse_container_name(container) do
    {brightness, color, bag} = container
    |> String.split(" ")
    |> List.to_tuple

    brightness <> " " <> color
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_rules("input.txt")
    |> BagRules.find_holder_count_of("shiny gold")
  end

  def solve2() do
    load_rules("input.txt")
    |> BagRules.find_contents_count_of("shiny gold")
  end

  defp load_rules(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> Enum.map(&BagRules.parse_rule/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
