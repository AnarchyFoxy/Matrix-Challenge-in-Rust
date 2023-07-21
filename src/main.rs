fn matrix_challenge(str_arr: &Vec<String>) -> String {
    let rows = str_arr.len();
    let cols = str_arr.iter().map(|s| s.len()).min().unwrap_or(0);
    let mut matrix = vec![vec![0; cols]; rows];

    // Convert the string matrix to a 2D integer matrix
    for i in 0..rows {
        let chars: Vec<char> = str_arr[i].chars().collect();
        for j in 0..cols {
            matrix[i][j] = chars[j].to_digit(10).unwrap() as u32;
        }
    }

    // Calculate the maximum area of the largest rectangular submatrix containing all 1's
    let max_area = max_rectangle_area(&matrix);

    max_area.to_string()
}

// Function to calculate the maximum area of the largest rectangular submatrix containing all 1's
fn max_rectangle_area(matrix: &Vec<Vec<u32>>) -> u32 {
    let rows = matrix.len();
    if rows == 0 {
        return 0;
    }
    let cols = matrix[0].len();
    let mut max_area = 0;

    let mut heights = vec![0; cols];

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 0 {
                heights[j] = 0;
            } else {
                heights[j] += matrix[i][j];
            }
        }
        max_area = max_area.max(largest_rectangle_area(&heights));
    }

    max_area
}

// Function to calculate the maximum area of the largest rectangular submatrix in a histogram
fn largest_rectangle_area(heights: &Vec<u32>) -> u32 {
    let n = heights.len();
    let mut max_area = 0;
    let mut stack = Vec::new();

    for i in 0..=n {
        while !stack.is_empty() && (i == n || heights[i] < heights[*stack.last().unwrap() as usize]) {
            let height = heights[stack.pop().unwrap() as usize];
            let width = match stack.last() {
                Some(&top) => i as u32 - top as u32 - 1,
                None => i as u32,
            };
            max_area = max_area.max(height * width);
        }
        stack.push(i as u32);
    }

    max_area
}

fn main() {
    // Test the function with the example inputs
    let str_arr1 = vec!["10100".to_string(), "10111".to_string(), "1111".to_string(), "10010".to_string()];
    let str_arr2 = vec!["1011".to_string(), "0011".to_string(), "0111".to_string(), "1111".to_string()];

    let output1 = matrix_challenge(&str_arr1);
    let output2 = matrix_challenge(&str_arr2);

    println!("{}", output1); // Output: "6"
    println!("{}", output2); // Output: "8"
}
