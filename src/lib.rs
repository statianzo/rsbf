#[allow(dead_code)]
mod rsbf {
    use std::collections::HashMap;

    fn jump_table(code: &str) -> HashMap<usize, usize> {
        let mut jumps = HashMap::new();
        let mut stack : Vec<usize> = Vec::new();
        for (index, c) in code.char_indices() {
            match c {
                '[' => stack.push(index),
                ']' => {
                    let open_index = stack.pop().expect("Missing opening bracket");
                    jumps.insert(index, open_index);
                    jumps.insert(open_index, index);
                }
                _ => {}
            }
        }

        jumps
    }

    pub fn run(code: &str) -> String {
        let mut output = String::new();
        let mut data: Vec<u8> = Vec::new();
        data.resize(30000, 0);
        let mut icode = 0;
        let mut idata = 0;
        let jumps = jump_table(code);
        while icode < code.len() {
            let cmd = &code[icode..icode + 1];
            match cmd {
                "<" => idata = idata - 1,
                ">" => idata = idata + 1,
                "+" => data[idata] = data[idata] + 1,
                "-" => data[idata] = data[idata] - 1,
                "." => output.push(data[idata] as char),
                "[" => if data[idata] == 0 {
                    icode = *jumps.get(&icode).expect("missing [ jump");
                },
                "]" => if data[idata] != 0 {
                    icode = *jumps.get(&icode).expect("missing ] jump");
                },
                _ => {}
            }
            icode = icode + 1;
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use rsbf::*;

    #[test]
    fn it_works() {
        let result = run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        assert_eq!(result, "Hello World!\n");
    }
}
