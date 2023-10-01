namespace AdventOfCode;

public class Day01 : BaseDay {
    private readonly string _input;

    public Day01() {
        _input = File.ReadAllText(InputFilePath);
    }

    public List<int> GetElfsCals() {
        var elfs = _input.Split(new string[] { Environment.NewLine + Environment.NewLine },
                               StringSplitOptions.RemoveEmptyEntries);

        var max_calories = new List<int>();

        foreach (var elf in elfs) {
            var current = 0;
            var cals = elf.Split('\n');

            foreach (var cal in cals) {
                current += int.Parse(cal);
            }

            max_calories.Add(current);
        }

        return max_calories;
    }

    public override ValueTask<string> Solve_1() {
        var answer = GetElfsCals().Max();

        return new ValueTask<string>($"{answer}");
    }

    public override ValueTask<string> Solve_2() {
        var cals = GetElfsCals();

        cals.Sort((a, b) => b.CompareTo(a));

        var answer = cals.Take(3).Sum();

        return new ValueTask<string>($"{answer}");
    }

}
