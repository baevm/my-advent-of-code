namespace AdventOfCode;

public struct MathMonkey
{
    public string first;
    public string second;
    public string operation;
}

public class Day21 : BaseDay
{
    private readonly string _input;

    public Day21()
    {
        _input = File.ReadAllText(InputFilePath);
    }


    public override ValueTask<string> Solve_1()
    {
        var lines = _input.Split("\n");

        var loneMonkeys = new Dictionary<string, double>();
        var mathMonkeys = new Dictionary<string, MathMonkey>();

        foreach (var line in lines)
        {
            var parseMonkeys = line.Split(": ");

            var monkeyName = parseMonkeys[0];
            var numbers = parseMonkeys[1];

            // check if monkey is lone
            var isOk = double.TryParse(numbers, out var num);

            // if parsed number successfully => monkey is lone
            // else not
            if (isOk)
            {
                loneMonkeys[monkeyName] = num;
            }
            else
            {
                // [0] = first monkey
                // [1] = operation
                // [2] = second monkey
                var op = numbers.Split(" ");

                MathMonkey monkey = new()
                {
                    first = op[0],
                    operation = op[1],
                    second = op[2],
                };

                mathMonkeys[monkeyName] = monkey;
            }
        }

        var n = mathMonkeys.Count;

        // max we need to go is equal to size of mathMonkeys
        // for every i start start calculating mathMonkeys from start
        // at the end of i, mathMonkeys should be empty 
        for (int i = 0; i < n; i++)
        {
            foreach (var kv in mathMonkeys)
            {
                if (loneMonkeys.TryGetValue(kv.Value.first, out var firstNum)
                    && loneMonkeys.TryGetValue(kv.Value.second, out var secondNum))
                {
                    var res = kv.Value.operation switch
                    {
                        "+" => firstNum + secondNum,
                        "-" => firstNum - secondNum,
                        "*" => firstNum * secondNum,
                        "/" => firstNum / secondNum,
                        _ => throw new Exception($"Failed to get op: {kv.Value.operation}"),
                    };

                    loneMonkeys[kv.Key] = res;
                    mathMonkeys.Remove(kv.Key);
                }
            }
        }

        var isFoundRoot = loneMonkeys.TryGetValue("root", out var answer);

        return new ValueTask<string>($"{answer}");
    }

    public override ValueTask<string> Solve_2()
    {
        var answer = 0;
        return new ValueTask<string>($"{answer}");
    }

}
