// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.

pub fn fibonacci(n: u32) -> u32 {
    let n = n as usize;
    let mut calculated_numbers: Vec<u32>= Vec::with_capacity(31);
    calculated_numbers.push(0);
    calculated_numbers.push(1);
    if calculated_numbers.get(n).is_some() {
        calculated_numbers[n]
    }
    else {
        let mut last_calculated = calculated_numbers.len()-1;
        while last_calculated != n  {
            calculated_numbers.push(calculated_numbers[last_calculated] + calculated_numbers[last_calculated-1]);
            last_calculated += 1 ;
        }
        return calculated_numbers[last_calculated];
    }
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
