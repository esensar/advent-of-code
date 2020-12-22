defmodule CombatGame do
  def get_winner_deck(decks) do
    play_decks(decks)
  end

  defp play_decks({[], p2_deck}), do: p2_deck
  defp play_decks({p1_deck, []}), do: p1_deck
  defp play_decks({p1_deck, p2_deck}) do
    p1_card = Enum.at(p1_deck, 0)
    p2_card = Enum.at(p2_deck, 0)
    if p1_card > p2_card do
      play_decks({
        Enum.drop(p1_deck, 1) ++ [p1_card, p2_card],
        Enum.drop(p2_deck, 1)
      })
    else
      play_decks({
        Enum.drop(p1_deck, 1),
        Enum.drop(p2_deck, 1) ++ [p2_card, p1_card]
      })
    end
  end
end

defmodule RecursiveCombatGame do
  def get_winner_deck(decks) do
    play_game(decks)
  end

  defp play_game(decks, previous_states \\ [])
  defp play_game({[], p2_deck}, _), do: {:p2, p2_deck}
  defp play_game({p1_deck, []}, _), do: {:p1, p1_deck}
  defp play_game({p1_deck, p2_deck}, previous_states) do
    if Enum.count(previous_states, fn {p1, p2} -> p1 == p1_deck or p2 == p2_deck end) > 0 do
      {:p1, p1_deck}
    else
      p1_card = Enum.at(p1_deck, 0)
      p2_card = Enum.at(p2_deck, 0)
      winner = if (Enum.count(p1_deck) - 1) >= p1_card and (Enum.count(p2_deck) - 1) >= p2_card do
        {p, deck} = play_game({Enum.take(Enum.drop(p1_deck, 1), p1_card), Enum.take(Enum.drop(p2_deck, 1), p2_card)})
        p
      else
        if p1_card > p2_card do
          :p1
        else
          :p2
        end
      end
      if winner == :p1 do
        play_game({
          Enum.drop(p1_deck, 1) ++ [p1_card, p2_card],
          Enum.drop(p2_deck, 1)
        }, previous_states ++ [{p1_deck, p2_deck}])
      else
        play_game({
          Enum.drop(p1_deck, 1),
          Enum.drop(p2_deck, 1) ++ [p2_card, p1_card]
        }, previous_states ++ [{p1_deck, p2_deck}])
      end
    end
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_decks("input.txt")
    |> CombatGame.get_winner_deck
    |> Enum.reverse
    |> Enum.with_index
    |> Enum.map(fn {x, index} -> x * (index + 1) end)
    |> Enum.sum
  end

  def solve2() do
    load_decks("input.txt")
    |> RecursiveCombatGame.get_winner_deck
    |> elem(1)
    |> Enum.reverse
    |> Enum.with_index
    |> Enum.map(fn {x, index} -> x * (index + 1) end)
    |> Enum.sum
  end

  defp load_decks(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.trim
    |> String.split("\n\n")
    |> Enum.map(&String.split(&1, "\n"))
    |> Enum.map(&Enum.drop(&1, 1))
    |> Enum.map(fn x -> Enum.map(x, &String.to_integer/1) end)
    |> List.to_tuple
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
