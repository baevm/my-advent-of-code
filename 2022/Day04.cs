namespace AdventOfCode;

public class Day04 : BaseDay {
    private readonly string _input;
    private readonly string[] ranges;

    public Day04() {
        _input = File.ReadAllText(InputFilePath);
        ranges = _input.Split('\n');
    }

    public bool IsFullyOverlap(int start1, int end1, int start2, int end2) {
        return start2 >= start1 && end2 <= end1;
    }

    public bool IsPartialOverlap(int start1, int end1, int start2, int end2) {
        return start2 <= end1 && start1 <= end2;
    }

    public int[] GetRange(string range) {
        var subranges = range.Split(',');

        var first = subranges[0];
        var second = subranges[1];

        var first_ranges = first.Split('-');
        var second_ranges = second.Split('-');

        var first_start = int.Parse(first_ranges[0]);
        var first_end = int.Parse(first_ranges[1]);
        var second_start = int.Parse(second_ranges[0]);
        var second_end = int.Parse(second_ranges[1]);

        return new int[4] { first_start, first_end, second_start, second_end };
    }

    public override ValueTask<string> Solve_1() {
        var answer = 0;

        foreach (var range in ranges) {
            var rangeList = GetRange(range);

            int first_start = rangeList[0];
            int first_end = rangeList[1];
            int second_start = rangeList[2];
            int second_end = rangeList[3];


            if (IsFullyOverlap(first_start, first_end, second_start, second_end) ||
                IsFullyOverlap(second_start, second_end, first_start, first_end)) {

                answer += 1;
            }
        }

        return new ValueTask<string>($"{answer}");
    }

    public override ValueTask<string> Solve_2() {
        var answer = 0;

        foreach (var range in ranges) {
            var rangeList = GetRange(range);

            int first_start = rangeList[0];
            int first_end = rangeList[1];
            int second_start = rangeList[2];
            int second_end = rangeList[3];


            if (IsPartialOverlap(first_start, first_end, second_start, second_end) ||
                IsPartialOverlap(second_start, second_end, first_start, first_end)) {

                answer += 1;
            }
        }

        return new ValueTask<string>($"{answer}");
    }

}
