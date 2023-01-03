use clap::Parser;

fn check_valid_params(check_param: &String, list_valid_params: Vec<&str>) -> bool{
    let mut valid: bool = false;

    for operation in list_valid_params{
        if check_param.to_string().trim() == operation.to_string() {
            valid = true;
        }
    }

    return valid;
}

fn check_operations(input: &String) -> bool{
    let operation_list = vec!["sum", "subtraction", "multiplication", "division"];

    return check_valid_params(input, operation_list);
}

fn resolve_operation(input_operation: String, list_number: Vec<i32>) -> i32{
    let mut result: i32 = 0;

    for x in list_number{
        if input_operation.to_string().trim() == "sum"{
            result += x;
        }

        if input_operation.to_string().trim() == "subtraction"{
            result -= x;
        }

        if input_operation.to_string().trim() == "multiplication"{
            if result == 0 {
                result = 1;
            } 
            result *= x;
        }

        if input_operation.to_string().trim() == "division"{
            if result == 0 {
                result = x;
            } else{
                result /= x;
            }
        }
    }

    return result;
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Calculate {
    /// operation available = sum, subtraction, multiplication, division
    #[clap(short = 'o', long)]
    operation: String,

    // Values  
    #[clap(short = 'v', long)]
    value: String
}

fn main() {
    let calculate_parse = Calculate::parse();
    let operation = calculate_parse.operation;
    let value = calculate_parse.value
    .split_whitespace()
    .map(|f| f.parse().unwrap()).collect();
   
    let valid_operation: bool = check_operations(&operation);
    
    if !valid_operation {
        println!("invalid params");
        return;
    }

    let result = resolve_operation(operation, value);

    println!("result = {}!", result);
}