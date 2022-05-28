use crate::structs;
use structs::Sample;

pub fn find_sample_b_by_question(question: i8, samples: &Vec<Sample>) -> Option<i8> {
    for sample in samples {
        if sample.blueprint == question {
            return Some(sample.index);
        }
    }

    return None;
}

pub fn create_samples() -> Vec<Sample> {
    return vec![
        Sample { blueprint: 3, index: 8 },
        Sample { blueprint: 8, index: 9 },
        Sample { blueprint: 1, index: 4 },
    ];
}

pub fn get_question() -> Option<i8> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return None;
    }

    let question = args[1].parse::<i8>();

    if question.is_err() {
        return None;
    }

    return Some(question.unwrap());
}
