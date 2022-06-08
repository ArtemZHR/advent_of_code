fn main() {
    let mut result_a = 0;
    let mut result_b = 0;

    include_str!("../input.txt")
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|str_expr| {
            let mut expr = str_expr
                .split("x")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let (l, w, h) = (expr[0], expr[1], expr[2]);
            expr.sort();
            result_a += (2 * l * w) + (2 * w * h) + (2 * h * l) + (expr[0] * expr[1]);
        });

    include_str!("../input.txt")
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|str_expr| {
            let mut expr = str_expr
                .split("x")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            expr.sort();
            result_b += (expr[0] + expr[0] + expr[1] + expr[1]) + (expr[0] * expr[1] * expr[2]);
        });
}
