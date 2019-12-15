use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Chemical {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub inputs: HashMap<Chemical, i64>,
    pub output: (i64, Chemical)
}

impl Recipe {
    pub fn from_string(string: String) -> Recipe {
        let inp_out: Vec<_> = string.split_terminator(" => ").collect();

        let out_split = inp_out[1].split_terminator(" ").collect::<Vec<_>>();

        let output = (out_split[0].parse::<i64>().unwrap(), Chemical{
            name: out_split[1].to_string()
        });

        let input_split = inp_out[0].split_terminator(", ").collect::<Vec<_>>();

        let inputs = input_split.iter().map(|chem| {
            let chem = chem.split(" ").collect::<Vec<_>>();

            (Chemical {
                name: chem[1].to_string(),
            }, chem[0].parse::<i64>().unwrap())
        }).collect::<HashMap<Chemical, i64>>();

        Recipe {
            inputs: inputs,
            output: output
        }
    }
}
