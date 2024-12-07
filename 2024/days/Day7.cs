using System;

namespace aoc_2024;

public class Day7 : ISolver
{
    private readonly string[] _input;

    public Day7()
    {
        var filePath = Path.GetFullPath("./data/7.txt");
        _input = File.ReadAllLines(filePath);
    }


    public void Solve_1()
    {

        var goodVals = new List<long>();

        foreach (var line in _input)
        {
            var task = line.Split(": ");

            var expected = long.Parse(task[0]);
            var nums = task[1].Split(" ").Select(long.Parse).ToArray();


            var isFound = backtrack_part1(0, expected, nums, 0);

            if (isFound)
            {
                goodVals.Add(expected);
            }
        }

        var res = goodVals.Sum();

        Console.WriteLine($"Task 1: {res}");
    }


    bool backtrack_part1(long currValue, long expectedValue, long[] nums, int idx)
    {
        if (idx >= nums.Length)
        {
            return currValue == expectedValue;
        }


        var sumRes = currValue + nums[idx];

        if (sumRes <= expectedValue)
        {
            if (backtrack_part1(sumRes, expectedValue, nums, idx + 1))
            {
                return true;
            }
        }

        var multiplyRes = currValue * nums[idx];

        if (multiplyRes <= expectedValue)
        {
            if (backtrack_part1(multiplyRes, expectedValue, nums, idx + 1))
            {
                return true;
            }
        }


        return false;
    }

    public void Solve_2()
    {
        var goodVals = new List<long>();

        foreach (var line in _input)
        {
            var task = line.Split(": ");

            var expected = long.Parse(task[0]);
            var nums = task[1].Split(" ").Select(long.Parse).ToArray();


            var isFound = backtrack_part2(0, expected, nums, 0);

            if (isFound)
            {
                goodVals.Add(expected);
            }
        }

        var res = goodVals.Sum();

        Console.WriteLine($"Task 2: {res}");
    }

    bool backtrack_part2(long currValue, long expectedValue, long[] nums, int idx)
    {
        if (idx >= nums.Length)
        {
            return currValue == expectedValue;
        }


        var concatRes = long.Parse(currValue.ToString() + nums[idx]);

        if (concatRes <= expectedValue)
        {
            if (backtrack_part2(concatRes, expectedValue, nums, idx + 1))
            {
                return true;
            }
        }

        var sumRes = currValue + nums[idx];

        if (sumRes <= expectedValue)
        {
            if (backtrack_part2(sumRes, expectedValue, nums, idx + 1))
            {
                return true;
            }
        }

        var multiplyRes = currValue * nums[idx];

        if (multiplyRes <= expectedValue)
        {

            if (backtrack_part2(multiplyRes, expectedValue, nums, idx + 1))
            {
                return true;
            }
        }


        return false;
    }
}
