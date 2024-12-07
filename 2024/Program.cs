namespace aoc_2024;
using System.CommandLine;

class Program
{
    static void Main(string[] args)
    {
        var dayOption = new Option<string>(
            name: "--day",
            description: "day to run"
        );

        var rootCommand = new RootCommand("advent of code 2024 runner");

        rootCommand.AddOption(dayOption);

        rootCommand.SetHandler(RunDay, dayOption);

        rootCommand.Invoke(args);
    }

    private static void RunDay(string day)
    {
        var days = new Dictionary<string, ISolver> {
            {"1", new Day1()},
            {"2", new Day2()},
            {"3", new Day3()},
            {"7", new Day7()},
        };

        var dayClass = days.TryGetValue(day, out ISolver? value) ? value : throw new ArgumentException($"Invalid day {day}");

        RunWithTime(dayClass.Solve_1, "1");
        RunWithTime(dayClass.Solve_2, "2");
    }

    private static void RunWithTime(Action method, string task)
    {
        var watch1 = System.Diagnostics.Stopwatch.StartNew();
        method();
        watch1.Stop();
        var elapsedMs = watch1.ElapsedMilliseconds;

        Console.WriteLine($"Task {task} took {elapsedMs} ms");
        Console.WriteLine();
    }
}