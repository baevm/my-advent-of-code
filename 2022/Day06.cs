namespace AdventOfCode;

public class Day06 : BaseDay {
    private readonly string _input;
    private readonly string[] rows;

    public Day06() {
        _input = File.ReadAllText(InputFilePath);
    }

    public int GetMarkerIndex(int length) {
        var index = 0;

        var hashset = new HashSet<char>();

        for (var i = 0; i < _input.Length; i++) {
            int j;

            for (j = 0; j < length; j++) {
                if (hashset.Contains(_input[i + j])) {
                    hashset.Clear();
                    break;
                }

                hashset.Add(_input[i + j]);
            }

            if (hashset.Count == length) {
                index = i + j;
                break;
            }
            else {
                hashset.Clear();
            }
        }

        return index;
    }

    public override ValueTask<string> Solve_1() {
        var answer = GetMarkerIndex(4);

        return new ValueTask<string>($"{answer}");
    }

    public override ValueTask<string> Solve_2() {
        var answer = GetMarkerIndex(14);

        return new ValueTask<string>($"{answer}");
    }
}
