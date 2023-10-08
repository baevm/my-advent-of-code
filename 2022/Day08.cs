namespace AdventOfCode;

public class Day08 : BaseDay {
    private readonly string _input;
    private readonly int[,] matrix;
    private readonly int m;
    private readonly int n;

    public Day08() {
        _input = File.ReadAllText(InputFilePath);

        var rows = _input.Split('\n');

        m = rows.Length;
        n = rows[0].Length;

        matrix = new int[m, n];

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                matrix[i, j] = int.Parse(rows[i][j].ToString());
            }
        }
    }

    public override ValueTask<string> Solve_1() {
        var answer = 0;

        // count trees around the edge
        answer += CountBorderValues(matrix);

        for (int i = 1; i < m - 1; i++) {
            for (int j = 1; j < n - 1; j++) {

                if (IsVisibleRight(matrix, i, j) ||
                    IsVisibleLeft(matrix, i, j) ||
                    IsVisibleUp(matrix, i, j) ||
                    IsVisibleDown(matrix, i, j)) {
                    answer += 1;
                }
            }
        }

        return new ValueTask<string>($"{answer}");
    }

    public static bool IsVisibleRight(int[,] heightMap, int row, int col) {
        int height = heightMap[row, col];
        int size = heightMap.GetLength(1);

        for (int c = col + 1; c < size; c++) {
            int other = heightMap[row, c];

            if (other >= height) {
                return false;
            }
        }

        return true;
    }

    public static bool IsVisibleLeft(int[,] heightMap, int row, int col) {
        int height = heightMap[row, col];

        for (int c = col - 1; c >= 0; c--) {
            int other = heightMap[row, c];

            if (other >= height) {
                return false;
            }
        }

        return true;
    }

    public static bool IsVisibleUp(int[,] heightMap, int row, int col) {
        int height = heightMap[row, col];

        for (int r = row - 1; r >= 0; r--) {
            int other = heightMap[r, col];

            if (other >= height) {
                return false;
            }
        }

        return true;
    }

    public static bool IsVisibleDown(int[,] heightMap, int row, int col) {
        int height = heightMap[row, col];
        int size = heightMap.GetLength(1);

        for (int r = row + 1; r < size; r++) {
            int other = heightMap[r, col];

            if (other >= height) {
                return false;
            }
        }

        return true;
    }

    static int CountBorderValues(int[,] matrix) {
        int borderValueCount = 0;

        for (int i = 0; i < matrix.GetLength(0); i++) {
            for (int j = 0; j < matrix.GetLength(1); j++) {
                if (IsOnBorder(matrix, i, j)) {
                    borderValueCount++;
                }
            }
        }

        return borderValueCount;
    }

    static bool IsOnBorder(int[,] matrix, int i, int j) {
        return i == 0 || j == 0 || i == matrix.GetLength(0) - 1 || j == matrix.GetLength(1) - 1;
    }

    public override ValueTask<string> Solve_2() {
        var answer = 0;

        for (int i = 1; i < m - 1; i++) {
            for (int j = 1; j < n - 1; j++) {
                var left = VisibleLeft(matrix, i, j);
                var right = VisibleRight(matrix, i, j);
                var up = VisibleUp(matrix, i, j);
                var down = VisibleDown(matrix, i, j);

                answer = Math.Max(answer, left * right * up * down);
            }
        }

        return new ValueTask<string>($"{answer}");
    }

    static int VisibleLeft(int[,] heightMap, int row, int col) {
        var answer = 0;

        int height = heightMap[row, col];

        for (int c = col - 1; c >= 0; c--) {
            int other = heightMap[row, c];

            answer += 1;

            if (other >= height) {
                break;
            }
        }

        return answer;
    }

    static int VisibleRight(int[,] heightMap, int row, int col) {
        var answer = 0;

        int height = heightMap[row, col];
        int size = heightMap.GetLength(1);

        for (int c = col + 1; c < size; c++) {
            int other = heightMap[row, c];

            answer += 1;

            if (other >= height) {
                break;
            }
        }

        return answer;
    }

    static int VisibleUp(int[,] heightMap, int row, int col) {
        var answer = 0;

        int height = heightMap[row, col];

        for (int r = row - 1; r >= 0; r--) {
            int other = heightMap[r, col];

            answer += 1;

            if (other >= height) {
                break;
            }
        }

        return answer;
    }

    static int VisibleDown(int[,] heightMap, int row, int col) {
        var answer = 0;

        int height = heightMap[row, col];
        int size = heightMap.GetLength(1);

        for (int r = row + 1; r < size; r++) {
            int other = heightMap[r, col];

            answer += 1;

            if (other >= height) {
                break;
            }
        }

        return answer;
    }
}
