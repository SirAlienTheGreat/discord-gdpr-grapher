use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let paths = fs::read_dir("/run/media/sir-alien-the-great/Barracuda 4TB/Discord GDPR data/messages").unwrap();
    let mut str_path :Vec<String> = vec![];


    for path in paths { // Convert path (:ReadDir) to :String
        if !path.as_ref().unwrap().path().display().to_string().contains("index"){
            str_path.push([path.unwrap().path().display().to_string(),"/messages.csv".to_string()].join(""))
        }

    }


    let mut date:Vec<String> = vec![];
    let mut messages:Vec<String> = vec![];


    for path in str_path { // For every messages file:

        // Read CSV file as string
        let mut file = File::open(path)
            .expect("Why are you trying to break my program?\nWhy do you hate me?");
        let mut randomstring = String::new();
        file.read_to_string(&mut randomstring)
            .expect("You picked the wrong path because you wanted to see my program fail, didn't you?");

        // We only care about the 2nd and 3rd row of each csv, so write new vectors with just that
        let mut comma_counter = 5;
        let mut in_quotes = false;
        let mut current_date = "".to_string();
        let mut current_string = "".to_string();

        for c in randomstring.chars(){ // For every character:
            if comma_counter == 0 {
                if c != ',' || in_quotes {
                    if c == '"'{
                        if !in_quotes{
                            in_quotes = true;
                        } else {
                            in_quotes = false;
                        }
                    } else {
                        current_string.push(c);
                    }
                }else{
                    comma_counter = 2;
                    date.push(current_date);
                    current_date = "".to_string();

                    messages.push(current_string);
                    current_string = "".to_string();
                }
            } else if c != ',' && comma_counter ==1 {
                current_date.push(c);
            } else if c == ','{
                comma_counter+=-1;
            }
        }

    }
    println!("Date: {:?}",date[1000]);
    println!("Message {:?}",messages[1000]);
}