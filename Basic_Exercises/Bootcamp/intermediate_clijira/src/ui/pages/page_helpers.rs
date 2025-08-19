
//use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    //todo!() // use the truncate_ellipse function from the ellipse crate
    let text_len = text.chars().count();
    match width {
        0 => return String::new(),
        1 | 2 | 3 => {return ".".repeat(width)},
        _ if text_len > width => {
            let mut result_str=String::new();

            for(i,c) in text.chars().enumerate() {
                if i >= width - 3 {
                    break;
                }
                result_str.push(c);
            }
            result_str.push_str("...");
            //println!("text_len: {}, width: {}, result_str: {}", text_len, width, result_str);
            result_str
            

        }
        _ => {
            let mut result = text.to_string();
            // If the text is shorter than the width, pad with spaces
            result.push_str(&" ".repeat(width - text_len));
            return result;
        }
        
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}
