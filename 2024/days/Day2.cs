using System;

namespace aoc_2024;

public class Day2 : ISolver
{
    private readonly string[] _input;

    public Day2()
    {
        var filePath = Path.GetFullPath("./data/2.txt");
        _input = File.ReadAllLines(filePath);
    }


    public void Solve_1()
    {
        var safeReports = 0;

        foreach (var line in _input)
        {
            var levels = line.Split(" ").Select(int.Parse).ToList();

            var isSafeInput = IsSafe(levels);

            if (isSafeInput)
            {
                safeReports += 1;
            }
        }

        Console.WriteLine($"Task 1: {safeReports}");
    }

    public bool IsSafe(List<int> levels)
    {
        var isIncreasing = levels[0] < levels[1];

        for (var i = 1; i < levels.Count; i++)
        {
            var prev = levels[i - 1];
            var curr = levels[i];

            if (Math.Abs(prev - curr) > 3 || prev == curr || (isIncreasing && curr < prev) || (!isIncreasing && curr > prev))
            {
                return false;
            }
        }

        return true;
    }

    public void Solve_2()
    {
        var safeReports = 0;

        foreach (var line in _input)
        {
            var tolerable = false;
            var levels = line.Split(" ").Select(int.Parse).ToList();

            for (var i = 0; i < levels.Count; i++)
            {
                var newLevels = levels.Take(i).Concat(levels.Skip(i + 1)).ToList();

                if (IsSafe(newLevels))
                {
                    tolerable = true;
                    break;
                }
            }

            if (IsSafe(levels) || tolerable)
            {
                safeReports += 1;
            }
        }

        Console.WriteLine($"Task 2: {safeReports}");
    }
}
