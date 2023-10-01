namespace AdventOfCode;

public class Day03 : BaseDay {
    private readonly string _input;
    private readonly Dictionary<string, int> priorities = new();
    private readonly string alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    private readonly string[] rucksacks;

    public Day03() {
        _input = File.ReadAllText(InputFilePath);

        for (int i = 0; i < alphabet.Length; i++) {
            priorities[alphabet[i].ToString()] = i + 1;
        }

        rucksacks = _input.Split('\n');
    }

    public override ValueTask<string> Solve_1() {
        var answer = 0;

        foreach (var rucksack in rucksacks) {
            var first = rucksack[..(rucksack.Length / 2)];
            var second = rucksack[(rucksack.Length / 2)..];

            var commonLetter = ""; // common letter between 2 halfs

            var hashset = first.ToHashSet();

            for (int i = 0; i < second.Length; i++) {
                if (hashset.Contains(second[i])) {
                    commonLetter = second[i].ToString();
                    break;
                }
            }

            answer += priorities[commonLetter];
        }

        return new ValueTask<string>($"{answer}");
    }

    public override ValueTask<string> Solve_2() {
        var answer = 0;

        for (int i = 0; i < rucksacks.Length - 2; i += 3) {
            var first = rucksacks[i];
            var second = rucksacks[i + 1];
            var third = rucksacks[i + 2];

            var firstDict = first.ToHashSet();
            var secondDict = second.ToHashSet();

            var commonLetter = ""; // common letter between 3 strings

            for (int k = 0; k < third.Length; k++) {
                if (firstDict.Contains(third[k]) && secondDict.Contains(third[k])) {
                    commonLetter = third[k].ToString();
                }
            }

            answer += priorities[commonLetter];
        }

        return new ValueTask<string>($"{answer}");
    }

}
