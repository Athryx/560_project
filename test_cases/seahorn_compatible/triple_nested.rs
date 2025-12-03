pub fn state_machine(a: i32, b: i32) -> i32 {
    let mut state = 0;
    let mut i = 0;

    while i < a {
        let mut j = 0;

        while j < b {
            // linear state update rule
            state = state + 2*i - j;
            j += 1;
        }

        i += 1;
    }

    state
}

fn main() {
    let _ = state_machine(8, 3);
}
