using System;

namespace aoc_2024;

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
        var leftPriorityQueue = new PriorityQueue<int, int>();
        var rightPriorityQueue = new PriorityQueue<int, int>();

        foreach (var line in _input)
        {
            var nums = line.Split("   ");

            var num1 = int.Parse(nums[0]);
            var num2 = int.Parse(nums[1]);

            leftPriorityQueue.Enqueue(num1, num1);
            rightPriorityQueue.Enqueue(num2, num2);
        }

        var res = 0;

        while (leftPriorityQueue.Count > 0)
        {
            var minFromLeft = leftPriorityQueue.Dequeue();
            var minFromRight = rightPriorityQueue.Dequeue();

            var distance = Math.Abs(minFromLeft - minFromRight);

            res += distance;
        }

        Console.WriteLine($"Task 1: {res}");
    }

    public void Solve_2()
    {
        var numsInLeft = new List<int>();
        var countInRight = new Dictionary<int, int>();

        foreach (var line in _input)
        {
            var nums = line.Split("   ");

            var num1 = int.Parse(nums[0]);

            numsInLeft.Add(num1);

            var num2 = int.Parse(nums[1]);

            if (countInRight.ContainsKey(num2))
            {
                countInRight[num2] += 1;
            }
            else
            {
                countInRight[num2] = 1;
            }
        }

        var res = 0;


        foreach (var numLeft in numsInLeft)
        {
            var isFound = countInRight.TryGetValue(numLeft, out var countOfLeftNumInRight);

            if (isFound)
            {
                var similarityScore = numLeft * countOfLeftNumInRight;
                res += similarityScore;
            }

        }

        Console.WriteLine($"Task 2: {res}");
    }
}
