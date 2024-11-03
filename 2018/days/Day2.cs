namespace aoc_2018;

public class Day2 : ISolver
{
    private readonly string[] _input;

    public Day2()
    {
        var filePath = Path.GetFullPath("./data/2.txt");

        _input = File.ReadAllLines(filePath);
    }

    struct IdsCounter()
    {
        public long TwoCount = 0;
        public long ThreeCount = 0;
    };

    public void Solve_1()
    {
        var idsCounter = new IdsCounter();

        foreach (var line in _input)
        {
            var letterCounter = line
                .GroupBy((s) => s)
                .Select(s => new { key = s.Key, count = s.Count() })
                .ToDictionary((g) => g.key, g => g.count);

            var isTwoFound = false;
            var isThreeFound = false;

            foreach (var kv in letterCounter)
            {
                if (kv.Value == 2 && !isTwoFound)
                {
                    isTwoFound = true;
                    idsCounter.TwoCount += 1;
                }

                if (kv.Value == 3 && !isThreeFound)
                {
                    isThreeFound = true;
                    idsCounter.ThreeCount += 1;
                }

                if (isTwoFound && isThreeFound)
                {
                    break;
                }
            }
        }

        long res = idsCounter.TwoCount * idsCounter.ThreeCount;

        Console.WriteLine($"Task 1: {res}");
    }

    public void Solve_2()
    {
        foreach (var line1 in _input)
        {
            foreach (var line2 in _input)
            {
                var diff = 0;

                for (int i = 0; i < line1.Length; i++)
                {
                    if (line1[i] != line2[i])
                    {
                        diff += 1;
                    }
                }

                if (diff == 1)
                {
                    var common = line1.Where((c, i) => c == line2[i]);

                    Console.WriteLine($"Task 2: {string.Concat(common)}");
                    return;
                }
            }
        }
    }
}
