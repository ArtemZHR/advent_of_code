fn main() {
    let str = include_str!("../input.txt").trim();

    let a = str.matches("(").count() as i32 - str.matches(")").count() as i32;

    let mut vec_str = str
        .split("")
        .collect::<Vec<&str>>();

    vec_str.remove(0);
    vec_str.remove(vec_str.len() - 1);

    let mut b = 0;

    for (i, char) in vec_str.into_iter().enumerate() {
        if b == -1 {
            b = i as i32;
            break;
        }
        if char == "(" { b += 1; } else { b -= 1; }
    }
}
