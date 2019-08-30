use permutohedron::Heap;

fn to_indices(text: &str, indices: &String) -> Vec<usize> {
    return text
        .chars()
        .rev()
        .map(|s| indices.find(s).unwrap() as usize)
        .collect::<Vec<usize>>();
}

fn to_numeric(text: &str, indices: &String, permutation: &Vec<usize>) -> u32 {
    let result = text
        .chars()
        .rev()
        .map(|s| indices.find(s).unwrap() as usize)
        .map(|i| permutation[i].to_string())
        .collect::<Vec<String>>()
        .concat();
    return result.parse::<u32>().unwrap();
}

fn get_value(operand: &Vec<usize>, index: usize, permutation: &Vec<usize>) -> usize {
    if index >= operand.len() {
        return 0;
    }

    return permutation[operand[index]];
}

fn check_equality(operands: &Vec<Vec<usize>>, sum: &Vec<usize>, permutation: &Vec<usize>, index: usize, carry: usize) -> bool {
    if index >= sum.len() {
        return true;
    }

    let digits: Vec<usize> = operands.iter().map(|o| get_value(&o, index, permutation)).collect();

    let column_sum: usize = digits.iter().sum();
    let result = column_sum + carry;

    if result % 10 == permutation[sum[index]] {
        return check_equality(&operands, &sum, &permutation, index + 1, result / 10);
    }
    return false;
}

fn main() {
    let operands = vec!["eight", "three", "nine"];
    let sum = "twenty";

    let mut numbers: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heap = Heap::new(&mut numbers);

    let mut chars: Vec<char> = (operands.join("") + sum).chars().collect();
    chars.sort();
    chars.dedup();

    let indices: String = chars.into_iter().collect();

    let rev_operands = operands.iter().map(|o| to_indices(o, &indices)).collect();
    let rev_sum = to_indices(sum, &indices);

    for permutation in heap {
        if check_equality(&rev_operands, &rev_sum, &permutation, 0, 0) {
            for operand in &operands {
                println!("{:10}", to_numeric(operand, &indices, &permutation));
            }
            println!("----------");
            println!("{:10}", to_numeric(sum, &indices, &permutation));
            println!("");
        }
    }
}
