namespace AdventOfCode;

public class Day02 : BaseDay
{
    private readonly string _input;

    const int DRAW = 3;
    const int WIN = 6;

    const string ROCK = "Rock";
    const string PAPER = "Paper";
    const string SCISSORS = "Scissors";

    readonly Dictionary<string, string> items = new(){
        // elfs
            {"A", ROCK},
            {"B", PAPER},
            {"C", SCISSORS},
        // my
            {"X", ROCK},
            {"Y", PAPER},
            {"Z", SCISSORS}
    };

    readonly Dictionary<string, int> scores = new(){
            {ROCK, 1},
            {PAPER, 2},
            {SCISSORS, 3}
        };

    // key=elfs, value=my
    readonly Dictionary<string, string> my_winning_combos = new(){
            {SCISSORS, ROCK},
            {PAPER, SCISSORS},
            {ROCK, PAPER},
         };

    // key=elfs, value=my
    readonly Dictionary<string, string> my_losing_combos = new(){
            {PAPER, ROCK},
            {ROCK, SCISSORS},
            {SCISSORS, PAPER},
         };

    public Day02()
    {
        _input = File.ReadAllText(InputFilePath);
    }

    public override ValueTask<string> Solve_1()
    {
        var resultScore = 0;
        var combinations = _input.Split('\n');

        foreach (var combo in combinations)
        {
            var elfs = combo[0].ToString();
            var my = combo[2].ToString();

            // Add score for picked item
            resultScore += scores[items[my]];

            // Check for draw
            if (items[elfs] == items[my])
            {
                resultScore += DRAW;
            }

            // Check if i win the round
            if (my_winning_combos[items[elfs]] == items[my])
            {
                resultScore += WIN;
            }
        }

        return new ValueTask<string>($"{resultScore}");
    }

    public override ValueTask<string> Solve_2()
    {
        var resultScore = 0;
        const string draw_char = "Y";
        const string win_char = "Z";
        const string lose_char = "X";

        var combinations = _input.Split('\n');

        foreach (var combo in combinations)
        {
            var elfs = combo[0].ToString();
            var end = combo[2].ToString();

            if (end == draw_char)
            {
                resultScore += DRAW;
                resultScore += scores[items[elfs]];
            }
            else if (end == win_char)
            {
                resultScore += WIN;
                resultScore += scores[my_winning_combos[items[elfs]]];
            }
            else if (end == lose_char)
            {
                resultScore += scores[my_losing_combos[items[elfs]]];
            }
        }

        return new ValueTask<string>($"{resultScore}");
    }

}
