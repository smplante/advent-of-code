pub const TEST: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1
";

pub const ACTUAL: &str = "Monkey 0:
Starting items: 84, 72, 58, 51
Operation: new = old * 3
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 7

Monkey 1:
Starting items: 88, 58, 58
Operation: new = old + 8
Test: divisible by 2
  If true: throw to monkey 7
  If false: throw to monkey 5

Monkey 2:
Starting items: 93, 82, 71, 77, 83, 53, 71, 89
Operation: new = old * old
Test: divisible by 7
  If true: throw to monkey 3
  If false: throw to monkey 4

Monkey 3:
Starting items: 81, 68, 65, 81, 73, 77, 96
Operation: new = old + 2
Test: divisible by 17
  If true: throw to monkey 4
  If false: throw to monkey 6

Monkey 4:
Starting items: 75, 80, 50, 73, 88
Operation: new = old + 3
Test: divisible by 5
  If true: throw to monkey 6
  If false: throw to monkey 0

Monkey 5:
Starting items: 59, 72, 99, 87, 91, 81
Operation: new = old * 17
Test: divisible by 11
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 6:
Starting items: 86, 69
Operation: new = old + 6
Test: divisible by 3
  If true: throw to monkey 1
  If false: throw to monkey 0

Monkey 7:
Starting items: 91
Operation: new = old + 1
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 5
";
