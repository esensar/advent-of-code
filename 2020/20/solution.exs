defmodule MonochromeImageScanner do
  defp image_string_to_binary(image_string) do
    image_string
    |> String.replace("#", "1")
    |> String.replace(".", "0")
    |> String.to_integer(2)
  end

  defp edge_representation({image_id, image_data}) do
    top_edge = List.first(image_data)
               |> image_string_to_binary
    bottom_edge = List.last(image_data)
               |> image_string_to_binary
    left_edge = Enum.map(image_data, &String.first/1)
                |> Enum.join
                |> image_string_to_binary
    right_edge = Enum.map(image_data, &String.last/1)
                |> Enum.join
                |> image_string_to_binary
    {image_id, MapSet.new([top_edge, bottom_edge, left_edge, right_edge])}
  end

  defp edge?({_, potential_image_edges}, all_images) do
    Enum.count(all_images, fn {_, edges} ->
      Enum.count(MapSet.intersection(potential_image_edges, edges)) > 2
    end) == 0
  end

  def find_edge_ids(images) do
    edged_images = Enum.map(images, &edge_representation/1)
    Enum.reduce(edged_images, [], fn potential_edge, acc ->
      if edge?(potential_edge, edged_images) do
        acc ++ [potential_edge]
      else
        acc
      end
    end)
    |> IO.inspect
    # Take only Ids
    |> Enum.map(&elem(&1, 0))
  end

  def parse_image(image_data) do
    image_id = Enum.at(image_data, 0)
               |> String.split(" ")
               |> Enum.at(1)
               |> String.replace(":", "")
               |> String.to_integer

    {image_id, Enum.drop(image_data, 1)}
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_images("input.txt")
    |> MonochromeImageScanner.find_edge_ids
    |> Enum.reduce(&Kernel.*/2)
  end

  # def solve2() do
  #   load_starting_numbers("input.txt")
  #   |> ElfMemoryGame.find_number_at(30000000)
  # end

  defp load_images(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.trim
    |> String.split("\n\n")
    |> Enum.map(&String.split(&1, "\n"))
    |> Enum.map(&MonochromeImageScanner.parse_image/1)
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

# IO.puts("Problem 2 solution: ")
# ProblemSolver.solve2()
# |> IO.inspect
