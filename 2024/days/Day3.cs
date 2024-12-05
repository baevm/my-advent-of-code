using System;
using System.Text.RegularExpressions;

namespace aoc_2024;

public class Day3 : ISolver
{
    private readonly string _input;

    public Day3()
    {
        var filePath = Path.GetFullPath("./data/3.txt");
        _input = File.ReadAllText(filePath);
    }


    public void Solve_1()
    {
        // matches mul(11,8)
        var mulRegex = new Regex(@"mul\(([0-9]{1,},[0-9]{1,})\)");

        // matches 11,8
        var instructionsRegex = new Regex(@"[0-9]*,[0-9]*");

        var mulGroups = mulRegex.Matches(_input);

        var res = 0;

        foreach (Match mul in mulGroups)
        {
            var instructions = instructionsRegex.Matches(mul.Value);
            var withoutComma = instructions.First().Value.Split(",");


            var first = int.Parse(withoutComma.First());
            var last = int.Parse(withoutComma.Last());


            res += first * last;
        }

        Console.WriteLine($"Task 1: {res}");
    }


    public void Solve_2()
    {
        // matches don't() | mul(11,8) | do()
        var mulRegex = new Regex(@"don't\(\)|do\(\)|mul\(([0-9]{1,},[0-9]{1,})\)");

        // matches 11,8
        var instructionsRegex = new Regex(@"[0-9]*,[0-9]*");

        var mulGroups = mulRegex.Matches(_input);

        var res = 0;

        var isDisabled = false;

        foreach (Match mul in mulGroups)
        {
            if (mul.Value == "don't()")
            {
                isDisabled = true;
                continue;
            }

            if (mul.Value == "do()")
            {
                isDisabled = false;
                continue;
            }

            if (isDisabled)
            {
                continue;
            }

            var instructions = instructionsRegex.Matches(mul.Value);
            var withoutComma = instructions.First().Value.Split(",");


            var first = int.Parse(withoutComma.First());
            var last = int.Parse(withoutComma.Last());

            res += first * last;
        }


        Console.WriteLine($"Task 2: {res}");
    }
}
