defmodule BusScheduleEstimator do

  def find_earliest_bus({timestamp, schedules}) do
    bus_id = Enum.find(schedules, fn x -> x != :x and Integer.mod(timestamp, x) == 0 end)

    if bus_id == nil do
      find_earliest_bus({timestamp + 1, schedules})
    else
      {timestamp, bus_id}
    end
  end

  # A bit of cheating and hints tells this is solvable by using Chinese remainder theorem
  #
  # Let n1, ..., nk be integers greater than 1, which are often called moduli or divisors.
  # Let us denote by N the product of the ni.
  #
  # The Chinese remainder theorem asserts that if the ni are pairwise coprime,
  # and if a1, ..., ak are integers such that 0 ≤ ai < ni for every i,
  # then there is one and only one integer x,
  # such that 0 ≤ x < N 
  # and the remainder of the Euclidean division of x by ni is ai for every i.
  def find_golden_timestamp({_, schedules}) do
    bus_offsets = schedules
    |> Enum.with_index
    |> Enum.filter(fn {x, _} -> x != :x end)

    # First we need to find product of all bus Ids (N - product of ni by theorem)
    product = bus_offsets
              |> Enum.map(&elem(&1, 0))
              |> Enum.reduce(&Kernel.*/2)

    # Theorem states if there are integers ai (which are remainders, or in our case offsets)
    # There is then only one x that produces these remainders which is our solution
    # X = product - product_offset
    #
    # We can now use theorem proof to calculate required X
    # We want to solve the system:
    # x ≡ a1 ( mod n1 ) 
    # x ≡ a2 ( mod n2 )
    #
    # Bézout's identity asserts the existence of two integers m1 and m2
    # m1 * n1 + m2 * n2 = 1
    #
    # The integers m1 and m2 may be computed by the extended Euclidean algorithm.
    #
    # A solution is given by
    # x = a1 * m2 * n2 + a2 * m1 * n1
    #
    # Indeed,
    # x = a1 * m2 * n2 + a2 * m1 * n1 
    #   = a1 * (1 − m1 * n1) + a2 * m1 * n1 
    #   = a1 + (a2 − a1) * m1 * n1
    #
    # implying that x ≡ a 1 ( mod n 1 )
    # The second congruence is proved similarly, by exchanging the subscripts 1 and 2.
    #
    # Based on this, solution may be found using this proof:
    #
    # The constructive existence proof shows that, in the case of two moduli,
    # the solution may be obtained by the computation of the Bézout coefficients 
    # of the moduli, followed by a few multiplications,
    # additions and reductions modulo n1,n2 
    # (for getting a result in the interval (0 , n1 * n2 − 1).
    # As the Bézout's coefficients may be computed with the extended Euclidean algorithm,
    # the whole computation, at most, has a quadratic time complexity 
    # of O((s1 + s2)^2), where si denotes the number of digits of ni.
    #
    # For more than two moduli, the method for two moduli allows the replacement 
    # of any two congruences by a single congruence modulo the product of the moduli.
    # Iterating this process provides eventually the solution with a complexity,
    # which is quadratic in the number of digits of the product of all moduli.
    # This quadratic time complexity does not depend on the order 
    # in which the moduli are regrouped.
    # One may regroup the two first moduli,
    # then regroup the resulting modulus with the next one, and so on.
    # This strategy is the easiest to implement,
    # but it also requires more computation involving large numbers.
    #
    # Another strategy consists in partitioning the moduli in pairs whose 
    # product have comparable sizes (as much as possible), applying, in parallel,
    # the method of two moduli to each pair,
    # and iterating with a number of moduli approximatively divided by two.
    # This method allows an easy parallelization of the algorithm.
    # Also, if fast algorithms (that is algorithms working in quasilinear time)
    # are used for the basic operations, this method provides an algorithm 
    # for the whole computation that works in quasilinear time.
    #
    # x ≡ 0 ( mod 3 ) 
    # x ≡ 3 ( mod 4 ) 
    # x ≡ 4 ( mod 5 )
    #
    # On the current example (which has only three moduli),
    # both strategies are identical and work as follows.
    #
    # Bézout's identity for 3 and 4 is
    #
    # 1 * 4 + ( − 1 ) * 3 = 1  --  m2 = 1, n2 = 4, m1 = -1, n1 = 3
    #
    # Putting this in the formula given for proving the existence gives
    #
    # 4 * 0 + ( − 3 ) * 3 = − 9  --  a1 = 0, m2 = 1, n2 = 4, a2 = 3, m1 = -1, n1 = 3
    #
    # for a solution of the two first congruences,
    # the other solutions being obtained by adding to −9 any multiple of 3 * 4 = 12.
    # One may continue with any of these solutions,
    # but the solution 3 = ( − 9 ) * 12 is smaller (in absolute value) 
    # and thus leads probably to an easier computation
    #
    # Bézout identity for 5 and 3 * 4 = 12 is
    #
    # 5 * 5 + ( − 2 ) * 12 = 1  --  m2 = 5, n2 = 5, m1 = -2, n1 = 12
    #
    # Applying the same formula again, we get a solution of the problem:
    #
    # 25 * 3 − 24 * 4 = − 21  --  a1 = 3, m2 = 5, n2 = 5, a2 = 4, m1 = -2, n1 = 12
    #
    # The other solutions are obtained by adding any multiple of 3 * 4 * 5 = 60,
    # and the smallest positive solution is ( − 21 ) + 60 = 39.
    #
    # Based on this, for each of bus entries, we need to calculate the following:
    #  * ai -> this is the offset
    #  * ni -> this is easily calculated by dividing product with bus id
    #  * mi -> this is calculated using extended euclidean algorithm
    #
    # Once we calculate all of them, we need to sum them up to get x from the proof
    product_offset = bus_offsets
                     |> Enum.map(fn {id, offset} ->
                       # ni is easily calculated
                       ni = div(product, id)
                       # Calculate ai * ni * mi
                       ni * find_mi(ni, id) * offset
                     end)
                     # Add together all solutions
                     |> Enum.sum
                     # Now we have the X, but we want the smallest positive one
                     |> Kernel.rem(product)

    product - product_offset
  end

  def find_mi(ni, n, x \\ 1) do
    case rem(x * ni, n) do
      1 -> x
      _ -> find_mi(ni, n, x + 1)
    end
  end

  def parse_schedule_estimate(estimate) do
    {timestamp, schedule, _} = estimate
                            |> String.split("\n")
                            |> List.to_tuple
    timestamp = String.to_integer(timestamp)
    schedule = schedule
               |> String.split(",")
               |> Enum.map(fn x -> if x == "x", do: :x, else: String.to_integer(x) end)

    {timestamp, schedule}
  end
end

defmodule ProblemSolver do
  def solve1() do
    {original_timestamp, schedules} = load_schedule_estimate("input.txt")
    {timestamp, bus_id} = BusScheduleEstimator.find_earliest_bus({original_timestamp, schedules})

    (timestamp - original_timestamp) * bus_id
  end

  def solve2() do
    load_schedule_estimate("input.txt")
    |> BusScheduleEstimator.find_golden_timestamp
  end

  defp load_schedule_estimate(filename) do
    File.read!(Path.expand(filename, __DIR__))
    |> BusScheduleEstimator.parse_schedule_estimate
  end
end

IO.puts("Problem 1 solution: ")
ProblemSolver.solve1()
|> IO.inspect

IO.puts("Problem 2 solution: ")
ProblemSolver.solve2()
|> IO.inspect
