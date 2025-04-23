// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::cmp::min;

//Recursion parameters
static CD : i64= 1;
static CI : i64 = 1;
static CR : i64 = 1;



#[tauri::command]
fn greet(input: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", input)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,process,test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn test(input: Vec<i64>) -> Vec<i64> {
   input 
}


#[tauri::command]
fn process(input : &str) -> Vec<(String,i64)>{
    let prova = vec![
        "ciao".to_string(),
        "cacca".to_string(),
        "puzza".to_string(),
        "palazina".to_string(),
        "palazzona".to_string(),
        "palladino".to_string(),
        "judas".to_string()
    ];

    let res =  prova.iter().map(|s|(s.clone(),edit_distance_w(input, s))).collect();
    println!("{input} -> {:?}",res);
    return res
}



fn edit_distance_w(s1 : &str, s2 : &str) -> i64{
    let rows = s1.len()+1;
    let columns = s2.len()+1;
    println!("Rows {rows}, Columns {columns}");
    let mut edit_distance_matrix : Vec<Vec<i64>> = vec![vec![-1;columns];rows];

    //Initializing the matrix 

    for i in 0..rows{
        edit_distance_matrix[i][0] = i as i64 * CD;
    }

    for j in 0..columns{
        edit_distance_matrix[0][j] = j as i64 * CI; 
    }

    let res = edit_distance_r(&mut edit_distance_matrix, rows, columns, s1, s2, rows -1, columns -1 );

    //println!("Edit distance is {res}");
    

    //print_edit_matrix(&edit_distance_matrix, rows, columns,s1,s2);

    return res
    
}

fn edit_distance_r(matrix : &mut Vec<Vec<i64>>,rows : usize, columns : usize, s1 : &str, s2 : &str, i : usize, j : usize) -> i64{
    if i == 0 || j == 0 {
        //Stop condition
        return matrix[i][j]
    }
    let epsilon = if s1.chars().nth(i-1) == s2.chars().nth(j-1) {0} else {1};
    let left = if matrix[i-1][j] == -1 {edit_distance_r(matrix, rows, columns, s1, s2, i-1, j)}else{matrix[i-1][j]};
    let down = if matrix[i][j-1] == -1 {edit_distance_r(matrix, rows, columns, s1, s2, i, j-1)}else{matrix[i][j-1]};
    let diagonal = if matrix[i-1][j-1] == -1 {edit_distance_r(matrix, rows, columns, s1, s2, i-1, j-1)}else{matrix[i-1][j-1]};
    matrix[i][j] =  min(min(left + CD, down + CI), diagonal + (CR * epsilon));
    return matrix[i][j]
    
}

//For debug purposes

fn print_edit_matrix(matrix : &Vec<Vec<i64>>, rows : usize, columns : usize, s1 : &str, s2 : &str){
    for i in 0..rows{
        if i == rows -1 {
            print!("-");
        }
        else{
            print!("{}",s1.chars().nth(rows -2 -i).unwrap());
        }
        for j in 0..columns{
            print!("\t{}",matrix[rows -1 - i][j]);
        }
        println!("");
    }
    for j in 0..columns{
        if j == 0{
            print!("\t-");
        }
        else{
            print!("\t{}",s2.chars().nth(j-1).unwrap());
        }
    }
    println!("\n-----");
}