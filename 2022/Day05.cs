using System.Text.RegularExpressions;

namespace AdventOfCode;

struct MoveRow {
    public int crateCount;
    public int from;
    public int to;

    public MoveRow(int c, int f, int t) {
        crateCount = c;
        from = f;
        to = t;
    }
}

public class Day05 : BaseDay {
    private readonly string _input;
    private readonly int cratesEndLine = 8;
    private readonly int movesStartLine = 10;
    private readonly string[] rows;
    private readonly string stackOfCratesInput = "";

    public Day05() {
        _input = File.ReadAllText(InputFilePath);

        rows = _input.Split('\n');

        // Create string of stack rows to parse
        for (int i = 0; i < cratesEndLine; i++) {
            stackOfCratesInput += rows[i] + "\n";
        }
    }

    public override ValueTask<string> Solve_1() {
        var stacks = ParseInput(stackOfCratesInput);

        // Start moving crates in stacks
        for (int i = movesStartLine; i < rows.Length; i++) {
            var moveRow = GetMoveRow(rows[i]);

            // Remove crate from stack and add to stack
            for (int c = 0; c < moveRow.crateCount; c++) {
                stacks[moveRow.to].Push(stacks[moveRow.from].Pop());
            }
        }

        var answer = "";

        foreach (var kvp in stacks) {
            answer += kvp.Value.First();
        }

        return new ValueTask<string>($"{answer}");
    }

    public override ValueTask<string> Solve_2() {
        var stacks = ParseInput(stackOfCratesInput);

        // Start moving crates in stacks
        for (int i = movesStartLine; i < rows.Length; i++) {
            var moveRow = GetMoveRow(rows[i]);

            var removedCrates = new List<char>();

            for (int c = 0; c < moveRow.crateCount; c++) {
                removedCrates.Add(stacks[moveRow.from].Pop());
            }

            for (int c = moveRow.crateCount - 1; c >= 0; c--) {
                stacks[moveRow.to].Push(removedCrates[c]);
            }
        }

        var answer = "";

        foreach (var kvp in stacks) {
            answer += kvp.Value.First();
        }

        return new ValueTask<string>($"{answer}");
    }

    static Dictionary<int, Stack<char>> ParseInput(string input) {
        Dictionary<int, Stack<char>> result = new();

        string[] lines = input.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var colCount = 1;

        for (int col = 0; col < lines[0].Length; col++) {
            Stack<char> letters = new();

            for (int row = 0; row < lines.Length; row++) {
                char c = lines[row].Length > col ? lines[row][col] : ' ';

                if (char.IsLetter(c)) {
                    letters.Push(c);
                }
            }

            if (letters.Count > 0) {
                // new Stack(myStack) reverses stack
                // we need it because we parse from top to bottom
                result.Add(colCount, new Stack<char>(letters));
                colCount += 1;
            }
        }

        return result;
    }

    static MoveRow GetMoveRow(string move) {
        var digits = Regex.Matches(move, @"\d+");

        var crateCount = int.Parse(digits[0].Value);
        var from = int.Parse(digits[1].Value);
        var to = int.Parse(digits[2].Value);

        return new MoveRow(crateCount, from, to);
    }
}
