using System;

namespace aoc_2018;

public class Day1 : ISolver
{
    private readonly string[] _input;

    public Day1()
    {
        var filePath = Path.GetFullPath("./data/1.txt");

        _input = File.ReadAllLines(filePath);
    }

    public void Solve_1()
    {
        var res = _input.Select(int.Parse).Aggregate((a, b) => a + b);

        Console.WriteLine($"Task 1: {res}");
    }

    public void Solve_2()
    {
        var seen = new HashSet<int>();
        var inputInts = _input.Select(int.Parse);

        var curr = 0;
        var idx = 0;

        while (true)
        {
            var num = inputInts.ElementAt(idx);
            var newCurr = curr + num;

            if (seen.Contains(newCurr))
            {
                Console.WriteLine($"Task 2: {newCurr}");
                return;
            }
            else
            {
                curr = newCurr;
            }

            seen.Add(curr);

            if (idx == inputInts.Count() - 1)
            {
                idx = 0;
            }
            else
            {
                idx += 1;
            }
        }
    }
}
