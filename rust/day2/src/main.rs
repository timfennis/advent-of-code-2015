fn main() {
    let contents = std::fs::read_to_string("input").expect("Failed to read input file");

    let dimensions: Vec<(u32, u32, u32)> = contents.split("\n").map(&line_to_dimension).collect();
    let total_paper_size = dimensions
        .iter()
        .fold(0u32, |a, b| a + calculate_paper_size(*b));
    let total_ribbon_size = dimensions
        .iter()
        .fold(0u32, |a, b| a + calculate_ribbon_size(*b));

    println!("Total size is: {total_paper_size}");
    println!("Total ribbon size is: {total_ribbon_size}");
}

fn line_to_dimension(line: &str) -> (u32, u32, u32) {
    let dimensions: Vec<u32> = line
        .split("x")
        .map(|x| x.parse::<u32>().expect("Unable to parse int"))
        .collect();
    return (dimensions[0], dimensions[1], dimensions[2]);
}

fn calculate_paper_size((w, h, l): (u32, u32, u32)) -> u32 {
    let sides = vec![l * w, w * h, h * l];
    let smallest = *sides.iter().min().unwrap();

    return sides.iter().fold(0, |t, a| t + 2 * a) + smallest;
}

fn calculate_ribbon_size((w, h, l): (u32, u32, u32)) -> u32 {
    let r1 = *vec![w+w+h+h, h+h+l+l, w+w+l+l].iter().min().unwrap();
    return r1 + w * h * l;
}

#[cfg(test)]
mod tests {
    use crate::{calculate_paper_size, calculate_ribbon_size};
    #[test]
    fn test_calculate_size() {
        let result = calculate_paper_size((2, 3, 4));
        assert_eq!(result, 58);
    }

    #[test]
    fn test_calculate_ribbon_size() {
        let result = calculate_ribbon_size((2, 3, 4));
        assert_eq!(result, 34);
    }
}
