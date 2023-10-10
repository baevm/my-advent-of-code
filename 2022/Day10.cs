namespace AdventOfCode;

public class Day10 : BaseDay {
    private readonly string _input;
    private readonly string[] operations;

    public Day10() {
        _input = File.ReadAllText(InputFilePath);
        operations = _input.Split('\n');
    }

    public override ValueTask<string> Solve_1() {
        var answer = 0;

        var currentSignal = 1;
        var currentCycle = 0;

        var needCycle = 20;

        for (int i = 0; i < operations.Length; i++) {
            var op = operations[i];

            currentCycle += 1;

            if (op.StartsWith("addx")) {

                currentCycle += 1;

                if (currentCycle >= needCycle) {
                    var strength = needCycle * currentSignal;

                    answer += strength;

                    System.Console.WriteLine(answer);
                    needCycle += 40;
                }

                var instructions = op.Split(" ");
                currentSignal += int.Parse(instructions[1]);
            }
        }


        return new ValueTask<string>($"{answer}");
    }

    public override ValueTask<string> Solve_2() {
        var answer = 1;

        return new ValueTask<string>($"{answer}");
    }

}
