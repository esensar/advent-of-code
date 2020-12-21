defmodule AllergenSeeker do
  def parse_food(food) do
    [[ _ | [ ingredients | [ allergens | _ ] ] ]] = Regex.scan(~r/([\ \w]*)\(([\w\ ,]*)\)/, food)
    ingredients = ingredients
                  |> String.split(" ")
                  |> Enum.map(&String.trim/1)
                  |> Enum.filter(fn x -> String.length(x) > 0 end)
    allergens = allergens
                |> String.replace("contains", "")
                |> String.split(",")
                |> Enum.map(&String.trim/1)
    {ingredients, allergens}
  end

  def compile_allergen_appearance_map(foods) do
    foods
    |> Enum.reduce(%{}, fn {ingredients, allergens}, acc ->
      allergen_map = Enum.reduce(allergens, acc, fn allergen, allergen_acc ->
        existing = Map.get(allergen_acc, allergen, MapSet.new(ingredients))
        Map.put(allergen_acc, allergen, MapSet.intersection(existing, MapSet.new(ingredients)))
      end)
      Map.merge(acc, allergen_map, fn _k, v1, v2 ->
        MapSet.intersection(v1, v2)
      end)
    end)
  end

  def count_allergen_free_appearances(foods) do
    allergenic_foods = compile_allergen_appearance_map(foods)
                       |> Enum.flat_map(fn {_allergen, ingredients} ->
                         ingredients
                       end)
                       |> MapSet.new

    Enum.map(foods, fn {ingredients, _} ->
      Enum.count(ingredients, fn ingredient -> ingredient not in allergenic_foods end)
    end)
    |> Enum.sum
  end

  def compile_canonical_dangerous_list(foods) do
    map = compile_allergen_appearance_map(foods)
    |> Enum.sort_by(fn {allergen, i} -> Enum.count(i) end)

    sure_matches = map
    |> Enum.reduce(%{}, fn {a, i}, acc ->
      existing = Enum.reduce(map, MapSet.new([]), fn {allergen, ingredients}, acc ->
        if allergen != a do
          MapSet.union(acc, ingredients)
        else
          acc
        end
      end)
      Map.put(acc, a, MapSet.difference(i, existing))
      end)

    map
    |> Enum.reduce(%{}, fn {a, i}, acc ->
      if Enum.count(Map.get(sure_matches, a, [])) > 0 do
        Map.put(acc, a, Map.get(sure_matches, a))
      else
        existing = Enum.reduce(acc, MapSet.new([]), fn {allergen, ingredients}, acc ->
          MapSet.union(acc, ingredients)
        end)
        Map.put(acc, a, MapSet.difference(i, existing))
      end
    end)
    |> Enum.map(fn {a, i} -> {Enum.at(i, 0), a} end)
    |> Enum.sort_by(fn {i, a} -> a end)
    |> Enum.map(&elem(&1, 0))
    |> Enum.join(",")
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_foods("input.txt")
    |> AllergenSeeker.count_allergen_free_appearances
  end

  def solve2() do
    load_foods("input.txt")
    |> AllergenSeeker.compile_canonical_dangerous_list
  end

  defp load_foods(filename) do
    File.stream!(Path.expand(filename, __DIR__))
    |> Enum.map(&String.trim/1)
    |> Enum.map(&AllergenSeeker.parse_food/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
