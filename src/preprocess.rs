use std::process::exit;

pub fn detect_and_trim_macros(contents: String) -> String{
    let len = contents.len();
    let mut i: usize = 0;
    let mut new_source_code = String::new();

    while i<len {
        let ch = contents.chars().nth(i).unwrap();

        if ch == '#'{
            println!("Macros not implemented yet");
            exit(1);
            i+=1;
        }else{
            new_source_code += ch.to_string().as_str();
            i+=1;
        }

    }


    new_source_code

    
}

pub fn trim_comments(contents: String)->String{
    let mut i: usize = 0;
    let mut new_source_code = String::new();

    let mut comments_mul = false;
    let mut comments_single = false;

    let len = contents.len();

    while i<len{
        let ch = contents.chars().nth(i).unwrap();
        let next_ch =
        if i+1<len{
            contents.chars().nth(i+1).unwrap()
        }else{
            '\0'
        };
        let checker = ch.to_string() + next_ch.to_string().as_str();

        if !comments_mul && !comments_single {
            
            match checker.as_str() {
                "/*"=>{
                    
                    comments_mul = true;
                    
                    i+=2;
                    continue;
                }
                "//"=>{

                    comments_single = true;

                    i+=2;
                    continue;
                }
                _=>{}
            }
            
            new_source_code += ch.to_string().as_str();
            i+=1;
        }else{
            if comments_mul {
                if checker == "*/"{
                    comments_mul = false;
                    comments_single = false;
                    i+=2;
                }else{
                    i+=1;
                }
            }

            if comments_single {
                if ch == '\n'{
                    comments_mul = false;
                    comments_single = false;
                    i+=1;
                }else{
                    i+=1;
                }
            }

        }

    }

    new_source_code
}