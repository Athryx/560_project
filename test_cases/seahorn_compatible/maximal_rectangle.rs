fn main () {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let result = maximal_rectangle(&matrix);
    println!("Maximal Rectangle Area: {}", result);
}

fn maximal_rectangle(matrix: &Vec<Vec<char>>) -> i32 
{
    let n = matrix.len();
    let m = matrix[0].len();
    // let mut heights = vec![vec![0; m]; n];
    let mut heights = Vec::with_capacity(n);
    let mut row = Vec::with_capacity(m);
    // let mut i=0;
    // let mut j=0;

    for _ in 0..m {
        row.push(0);
    };

    for i in 0..n {
        for j in 0..m {
            row[j] = if matrix[i][j] == '1' { row[j] + 1 } else { 0 };
        };
        heights.push(row.clone());
    }
        
    let mut max_area = 0;

    for i in 0..n {
        let mut stack = Vec::new();
        
        for j in 0..m+1 {
            let h = if j == m { 0 } else { heights[i][j] };

            while stack.len() != 0 && heights[i][stack[stack.len()-1]] > h {
                let height = heights[i][stack.pop().unwrap()];
                let width = if stack.len() == 0 { j } else { j - stack[stack.len()-1] - 1 };
                max_area = max_area.max(height * width);
            }
            stack.push(j);
        }
    }
    
    max_area as i32
}