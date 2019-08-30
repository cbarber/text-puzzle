use permutohedron::Heap;

const INDICES: &str = "teyhnigrw";

fn to_indices(text: &str) -> Vec<usize> {
    return text.split("").filter(|&s| s != "").map(|s| INDICES.find(s).unwrap() as usize).collect::<Vec<usize>>();
}

fn to_numeric(indices: &Vec<usize>, permutation: &Vec<u8>) -> u32 {
    let text: String = indices.iter().map(|&i| permutation[i].to_string()).collect::<Vec<String>>().concat();
    return text.parse::<u32>().unwrap();
}

fn has_short_circuit_inequality(permutation: &Vec<u8>, eight: &Vec<usize>, three: &Vec<usize>, nine: &Vec<usize>, twenty: &Vec<usize>, shift: usize) -> bool {
    let sum = permutation[eight[eight.len() - shift]] 
        + permutation[three[three.len() - shift]] 
        + permutation[nine[nine.len() - shift]];
    let digit = sum % 10;
    return digit != permutation[twenty[twenty.len() - shift]];
}

fn main() {
    let eight: Vec<usize> = to_indices("eight");
    let three: Vec<usize> = to_indices("three");
    let nine: Vec<usize> = to_indices("nine");
    let twenty: Vec<usize> = to_indices("twenty");

    let mut numbers: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heap = Heap::new(&mut numbers);

    for permutation in heap {
        if has_short_circuit_inequality(&permutation, &eight, &three, &nine, &twenty, 1) {
            continue;
        }

        if to_numeric(&eight, &permutation) + to_numeric(&three, &permutation) + to_numeric(&nine, &permutation) == to_numeric(&twenty, &permutation) {
            println!("{:8}", to_numeric(&eight, &permutation));
            println!("{:8}", to_numeric(&three, &permutation));
            println!("{:8}", to_numeric(&nine, &permutation));
            println!("--------");
            println!("{:8}", to_numeric(&twenty, &permutation));
            println!("");
        }
    }
}
