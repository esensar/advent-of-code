defmodule Passport do
  @mandatory_fields [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid]
  @extra_fields [:cid]
  @all_fields @mandatory_fields ++ @extra_fields
  @valid_eye_colors ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

  def parse_batch_file(passports) do
    passports
    |> Enum.map(&parse_passport_entry/1)
  end

  def valid_format?(passport) do
    passport_keys = passport
    |> Enum.map(&Kernel.elem(&1, 0))

    @mandatory_fields
    |> Enum.map(&Kernel.in(&1, passport_keys))
    |> Enum.reduce(&Kernel.and/2)
  end

  def valid_data?(passport) do
    valid_fields = passport
                   |> Enum.map(&valid_field?/1)
                   |> Enum.reduce(&Kernel.and/2)

    valid_fields and valid_format?(passport)
  end

  defp valid_field?({:byr, value}) do
    value = String.to_integer(value)
    value >= 1920 and value <= 2002
  end
  defp valid_field?({:iyr, value}) do
    value = String.to_integer(value)
    value >= 2010 and value <= 2020
  end
  defp valid_field?({:eyr, value}) do
    value = String.to_integer(value)
    value >= 2020 and value <= 2030
  end
  defp valid_field?({:hgt, value}) do
    rgx = ~r/^([[:digit:]]+)(cm|in)$/
    if String.match?(value, rgx) do
      case Regex.scan(rgx, value) do
        [[_, number, "cm"]] ->
          number = String.to_integer(number)
          number >= 150 and number <= 193
        [[_, number, "in"]] ->
          number = String.to_integer(number)
          number >= 59 and number <= 76
        _ -> false
      end
    else
      false
    end
  end
  defp valid_field?({:hcl, value}), do: String.match?(value, ~r/^#[[:xdigit:]]{6}$/)
  defp valid_field?({:ecl, value}), do: value in @valid_eye_colors
  defp valid_field?({:pid, value}), do: String.match?(value, ~r/^[[:digit:]]{9}$/)
  defp valid_field?({_key, _value}), do: true

  defp parse_passport_entry(entry) do
    entry
    |> String.split(~r"\n|\ ")
    |> Enum.map(&parse_passport_entry_field/1)
  end

  defp parse_passport_entry_field(field) do
    split = field
    |> String.split(":")
    {String.to_atom(Enum.at(split, 0)), Enum.at(split, 1)}
  end
end

defmodule ProblemSolver do
  def solve1() do
    load_passports("input.txt")
    |> Enum.count(&Passport.valid_format?/1)
  end

  def solve2() do
    load_passports("input.txt")
    |> Enum.count(&Passport.valid_data?/1)
  end

  defp load_passports(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> String.split("\n\n")
    |> Passport.parse_batch_file
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
