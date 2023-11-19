namespace AdventOfCode;

public struct Monkey
{
    public int Id;
    public List<double> StartingItems;
    public Operation Operation;
    public TestCondition Test;
}

public struct Operation
{
    public string symbol;
    public int value;
}

public struct TestCondition
{
    public int Condition;
    public int TrueBranch;
    public int FalseBranch;
}


public class Day11 : BaseDay
{
    private readonly string _input;
    private Dictionary<int, Monkey> monkeys;

    public Day11()
    {
        _input = File.ReadAllText(InputFilePath);
    }

    static Dictionary<int, Monkey> ParseMonkeys(string input)
    {
        Dictionary<int, Monkey> monkeys = new();

        string[] lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        int i = 0;
        while (i < lines.Length)
        {
            if (lines[i].StartsWith("Monkey"))
            {
                Monkey monkey = new()
                {
                    Id = int.Parse(lines[i].Substring(lines[i].IndexOf(' ') + 1, lines[i].IndexOf(':') - lines[i].IndexOf(' ') - 1))
                };

                i++; // Move to the next line
                while (i < lines.Length && !lines[i].StartsWith("Monkey"))
                {
                    if (lines[i].StartsWith("  Starting items:"))
                    {
                        monkey.StartingItems = lines[i]
                            .Substring(lines[i].IndexOf(':') + 1)
                            .Split(',', StringSplitOptions.RemoveEmptyEntries)
                            .Select(s => double.Parse(s.Trim()))
                            .ToList();
                    }
                    else if (lines[i].StartsWith("  Operation:"))
                    {
                        Operation op = new();

                        var oper = lines[i].Split(" ");
                        var znak = oper[6];
                        var opNum = oper[7];

                        if (opNum == "old")
                        {
                            op.symbol = "**";
                            op.value = 1;
                        }
                        else
                        {
                            op.symbol = znak;
                            op.value = int.Parse(opNum);
                        }

                        monkey.Operation = op;
                    }
                    else if (lines[i].StartsWith("  Test:"))
                    {
                        TestCondition test = new();
                        test.Condition = int.Parse(lines[i].Substring(lines[i].IndexOf(':') + 15).Trim());

                        i++; // Move to the next line

                        if (lines[i].StartsWith("    If true:"))
                        {
                            test.TrueBranch = int.Parse(lines[i][(lines[i].LastIndexOf(' ') + 1)..]);
                        }

                        i++;

                        if (lines[i].StartsWith("    If false:"))
                        {
                            test.FalseBranch = int.Parse(lines[i][(lines[i].LastIndexOf(' ') + 1)..]);
                        }


                        monkey.Test = test;
                    }

                    if (i < lines.Length)
                        i++; // Move to the next line
                }

                // Add the monkey to the list
                monkeys.Add(monkey.Id, monkey);
            }
            else
            {
                i++; // Move to the next line if it doesn't start with "Monkey"
            }
        }

        return monkeys;
    }

    public override ValueTask<string> Solve_1()
    {
        monkeys = ParseMonkeys(_input);
        var inspects = new Dictionary<int, int>();

        for (var i = 0; i < 20; i++)
        {
            foreach (var monkey in monkeys)
            {
                var operation = monkey.Value.Operation;
                var itemsToRemove = new List<double>();

                foreach (var item in monkey.Value.StartingItems)
                {
                    itemsToRemove.Add(item);
                    double newItem = 0;

                    // Get new monkey item
                    if (operation.symbol == "+")
                    {
                        newItem = item + operation.value;
                    }
                    else if (operation.symbol == "*")
                    {
                        newItem = item * operation.value;
                    }
                    else if (operation.symbol == "**")
                    {
                        newItem = Math.Pow(item, 2);
                    }

                    inspects[monkey.Key] = inspects.GetValueOrDefault(monkey.Key, 0) + 1;
                    // Get borted item level
                    newItem = Math.Floor(newItem / 3);

                    // Test monkey item level
                    if (newItem % monkey.Value.Test.Condition == 0)
                    {
                        monkeys[monkey.Value.Test.TrueBranch].StartingItems.Add(newItem);
                    }
                    else
                    {
                        monkeys[monkey.Value.Test.FalseBranch].StartingItems.Add(newItem);
                    }
                }

                // Remove used items from monkey
                foreach (var item in itemsToRemove)
                {
                    monkeys[monkey.Key].StartingItems.Remove(item);
                }
            }
        }


        System.Console.WriteLine("After: ");
        foreach (var monkey in monkeys)
        {
            Console.WriteLine($"Monkey {monkey.Key}:");
            Console.WriteLine($"  Starting items: {string.Join(", ", monkey.Value.StartingItems)}");
            Console.WriteLine($"  Operation: {monkey.Value.Operation.symbol} {monkey.Value.Operation.value}");
            Console.WriteLine($"  Test: {monkey.Value.Test.Condition}");
            Console.WriteLine($"    If true: {monkey.Value.Test.TrueBranch}");
            Console.WriteLine($"    If false: {monkey.Value.Test.FalseBranch}");
            Console.WriteLine();
        }

        var inspectValues = new List<int>();

        inspectValues.Sort((x, y) => y - x);

        var businessLevel = inspectValues[0] * inspectValues[1];

        return new ValueTask<string>($"{businessLevel}");
    }

    public override ValueTask<string> Solve_2()
    {
        monkeys = ParseMonkeys(_input);
        var inspects = new Dictionary<int, int>();
        double supermodulo = 1;

        foreach (var monk in monkeys)
        {
            supermodulo *= monk.Value.Test.Condition;
        }

        for (var i = 0; i < 10000; i++)
        {
            foreach (var monkey in monkeys)
            {
                var operation = monkey.Value.Operation;
                var itemsToRemove = new List<double>();

                foreach (var item in monkey.Value.StartingItems)
                {
                    itemsToRemove.Add(item);
                    double newItem = 0;

                    // Get new monkey item
                    if (operation.symbol == "+")
                    {
                        newItem = item + operation.value;
                    }
                    else if (operation.symbol == "*")
                    {
                        newItem = item * operation.value;
                    }
                    else if (operation.symbol == "**")
                    {
                        newItem = Math.Pow(item, 2);
                    }

                    inspects[monkey.Key] = inspects.GetValueOrDefault(monkey.Key, 0) + 1;
                    // Get borted item level
                    newItem = Math.Floor(newItem % supermodulo);

                    // Test monkey item level
                    if (newItem % monkey.Value.Test.Condition == 0)
                    {
                        monkeys[monkey.Value.Test.TrueBranch].StartingItems.Add(newItem);
                    }
                    else
                    {
                        monkeys[monkey.Value.Test.FalseBranch].StartingItems.Add(newItem);
                    }
                }

                // Remove used items from monkey
                foreach (var item in itemsToRemove)
                {
                    monkeys[monkey.Key].StartingItems.Remove(item);
                }
            }
        }


        System.Console.WriteLine("After: ");
        foreach (var monkey in monkeys)
        {
            Console.WriteLine($"Monkey {monkey.Key}:");
            Console.WriteLine($"  Starting items: {string.Join(", ", monkey.Value.StartingItems)}");
            Console.WriteLine($"  Operation: {monkey.Value.Operation.symbol} {monkey.Value.Operation.value}");
            Console.WriteLine($"  Test: {monkey.Value.Test.Condition}");
            Console.WriteLine($"    If true: {monkey.Value.Test.TrueBranch}");
            Console.WriteLine($"    If false: {monkey.Value.Test.FalseBranch}");
            Console.WriteLine();
        }

        var inspectValues = new List<int>();

        inspectValues.Sort((x, y) => y - x);

        long businessLevel = (long)inspectValues[0] * inspectValues[1];

        return new ValueTask<string>($"{businessLevel}");
    }

}
