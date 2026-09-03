use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

//function used to view the tasks in the file, returns true if it executes successfully, if it fails it returns a false
fn viewer(file_name:&str)->bool{
    let mut file_object = match fs::OpenOptions::new().read(true).open(file_name){//trying to open an existing file
        Ok(file)=>file,
        Err(_)=>{
        println!("There was an error in opening the file. Will create a new file for use");//if file doesnt exist, tries creating a new file
        match fs::OpenOptions::new().create(true).append(true).open(file_name){
        Ok(file)=>file,
        Err(_)=>{println!("There was an error in creating the file.");return false;},
        };//creating the file if it doesnt exist, and immediately exiting as there is nothing to view
        return false;},
    };

    let file_size = match file_object.metadata(){
        Ok(metadata )=>metadata.len(),
        Err(_)=>1,
    };
    if file_size==0{println!("No Tasks are availabe");return false;}//prints this if file is empty
    
    let mut file_content = String::new();
    match file_object.read_to_string(&mut file_content){
        Ok(_)=>{},
        Err(_)=>{println!("There was an error in reading the file");return false;},
    };//reading the file to a string
    for i in file_content.lines(){
        println!("{}",i);
    }//iterating through and printing the tasks
    return true;
}

fn adder(file_name:&str)->bool{
    let mut file_object =  match fs::OpenOptions::new().append(true).create(true).open(file_name){//opening the file, if it doest exist, creates one
        Ok(file)=>file,
        Err(_)=>{println!("Error in opening the file.");return false;},
    };
    println!("Enter the task you would like to add in the format given below");
    println!("Task number - Task - [DD/MM/YY]");
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_)=>{},
        Err(_)=>{println!("Error in taking input");return false;},
    };//handling input failure
    let valid = validity_checker(&input);//checking if the given task is valid or not
    if valid==false {println!("Invalid format of task");return false;}
    match file_object.write_all(input.as_bytes()){//writes to the file, return false if it cannot write for some reason
        Ok(_)=>{},
        Err(_)=>{println!("Error in writing to the file.");return false;}
    };
    return true;  
   }

fn deleter(file_name:&str)->bool{
    let mut tasks = String::new();
    let mut input = String::new();
    let mut individual_tasks : Vec<&str> = Vec::new();
    let mut file_object = match fs::OpenOptions::new().read(true).write(true).open(file_name){
        Ok(file)=>file,
        Err(_)=>{println!("Error in opening the file.");return false;}
    };//opening the file in read mode
    let file_size = match file_object.metadata(){
        Ok(metadata )=>metadata.len(),
        Err(_)=>1,
    };
    if file_size==0{println!("No Tasks are availabe");return false;}
    
    match file_object.read_to_string(&mut tasks){
        Ok(_)=>{},
        Err(_)=>{println!("Error in reading the file.");return false;},
    };//reading the file into a string

    println!("Enter the task number that you would like to delete. ");
    match io::stdin().read_line(&mut input){
        Ok(_)=>{},
        Err(_)=>{println!("Error in taking input.");return false;}
    };//taking input to delete task

    let input : u32 = match input.trim().parse(){
        Ok(x)=>x,
        Err(_)=>{println!("You did not enter a number.");return false;},
    };//checking if the input is valid

    let individual_tasks:  Vec<&str> = {
    for task in tasks.lines(){
        let individual_task_in_chars: Vec<char> = task.chars().collect();
        if individual_task_in_chars[0].to_digit(10).expect("Should never happen")==input{
            continue;
        }
        else{
        individual_tasks.push(task);}
        
    }//pushing each individual task into a vector of tasks unless task number matches input given for deletion
    //then returns this vector of tasks (which deleted all the unneeded tasks) for us to use
    individual_tasks
    };
    let mut file_object = match File::create(file_name){//this is to reset the file to zero and overwrite it
        Ok(file)=>file,
        Err(_)=>{println!("Error in overwriting the file.");return false;},
    };
    
    for i in individual_tasks{
        match file_object.write_all(i.as_bytes()){
            Ok(_)=>{},
            Err(_)=>{println!("Error in writing task to the file, exiting");return false;}
        }
    }
    return true;
}

fn runner(choice:u32,file_name:&str){
    if choice == 1{viewer(file_name);}
    else if choice == 2{adder(file_name);}
    else {deleter(file_name);}
}

//checks if the string written in adder() function is in the valid task format
fn validity_checker(task:&str)->bool{
    let task_in_vector :Vec<&str> = task.split_whitespace().collect();//takes the task str into a vector of strs, removing whitespaces

    //this checks whether the task number is entered, if not returns false
    match task_in_vector[0].parse::<usize>(){
        Ok(_)=>{},
        Err(_)=>{return false;}
    }

    let date:Vec<char> = task_in_vector[task_in_vector.len()-1].chars().collect();
    let result = date_validity_checker(date);
    result
}

//checks if the date given in adder() i correct or not, is called by validity_checker()
fn date_validity_checker(date:Vec<char>)->bool{
    if date.len()!=10{return false;}//checks if date is of valid length
    if date[0]!='['||date[9]!=']'{return false;}//checks if brackets are valid
    if date[3]!='/'||date[6]!='/'{return false;}//checks if the slashes dividing the dates are given correctly

    //checks if the days are valid
    {let mut d0 = match date[1].to_digit(10){
        Some(x)=>x,
        None=>return false,};

    d0 = d0*10;
    let d1 = match date[2].to_digit(10){
        Some(x)=>x,
        None=>return false,};

    d0 = d0+d1;
    if d0>31{return false;}}
    
    //checks if the month is valid
    {   let mut m0 = match date[4].to_digit(10){
        Some(x)=>x,
        None=>return false,};
        m0 = m0*10;
        let m1 = match date[1].to_digit(10){
        Some(x)=>x,
        None=>return false,};
        m0 = m0+m1;
        if m0>12{return false;} 
    }

    //checks if the year is valid
    {   let _y0 = match date[7].to_digit(10){
        Some(x)=>x,
        None=>return false,};
        let _y1 = match date[8].to_digit(10){
        Some(x)=>x,
        None=>return false,};
    }
    return true;
}
fn main(){
    let mut choice : u32 = 1;
    let file_name = "tasks.txt";
    while choice!=0 {
        println!("Enter 1 to view tasks, Enter 2 to add tasks, Enter 3 to delete, Enter 0 to exit");
        let mut input : String = String::new();
        //--------------------------------------------------------------------------------------
        match io::stdin().read_line(&mut input){//taking input
            Ok(_)=>{},
            Err(_)=>{println!("There is an error in taking the input. Sorry.");break;}//error handling for taking input
        };
        //--------------------------------------------------------------------------------------
        choice = match input.trim().parse(){
            Ok(x)=>{
                    if x==0 {println!("Exiting ..");break;}
                    if x>3{println!("Please enter a valid option");continue;}
                    else{x}},
            Err(_)=>{println!("Please enter a valid option");continue;}
        };//parsing inputs and handling invalid inputs
        //--------------------------------------------------------------------------------------
        runner(choice,file_name);
    }


}