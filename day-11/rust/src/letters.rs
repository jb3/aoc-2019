use std::collections::HashMap;

fn get_letters() -> HashMap<Vec<Vec<bool>>, char> {
    let letters: HashMap<Vec<Vec<bool>>, char> = [
        (
            vec![
                vec![false, true, true, true, true, true],
                vec![true, false, false, true, false, false],
                vec![true, false, false, true, false, false],
                vec![false, true, true, true, true, true],
                vec![false, false, false, false, false, false],
            ],
            'A',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![true, false, true, false, false, true],
                vec![true, false, true, false, false, true],
                vec![false, true, false, true, true, false],
                vec![false, false, false, false, false, false],
            ],
            'B',
        ),
        (
            vec![
                vec![false, true, true, true, true, false],
                vec![true, false, false, false, false, true],
                vec![true, false, false, false, false, true],
                vec![false, true, false, false, true, false],
                vec![false, false, false, false, false, false],
            ],
            'C',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![true, false, true, false, false, true],
                vec![true, false, true, false, false, true],
                vec![true, false, false, false, false, true],
                vec![false, false, false, false, false, false],
            ],
            'E',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![true, false, true, false, false, false],
                vec![true, false, true, false, false, false],
                vec![true, false, false, false, false, false],
                vec![false, false, false, false, false, false],
            ],
            'F',
        ),
        (
            vec![
                vec![false, true, true, true, true, false],
                vec![true, false, false, false, false, true],
                vec![true, false, false, true, false, true],
                vec![false, true, false, true, true, true],
                vec![false, false, false, false, false, false],
            ],
            'G',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![false, false, true, false, false, false],
                vec![false, false, true, false, false, false],
                vec![true, true, true, true, true, true],
                vec![false, false, false, false, false, false],
            ],
            'H',
        ),
        (
            vec![
                vec![false, false, false, false, true, false],
                vec![false, false, false, false, false, true],
                vec![true, false, false, false, false, true],
                vec![true, true, true, true, true, false],
                vec![false, false, false, false, false, false],
            ],
            'J',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![false, false, true, false, false, false],
                vec![false, true, false, true, true, false],
                vec![true, false, false, false, false, true],
                vec![false, false, false, false, false, false],
            ],
            'K',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![false, false, false, false, false, true],
                vec![false, false, false, false, false, true],
                vec![false, false, false, false, false, true],
                vec![false, false, false, false, false, false],
            ],
            'L',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![true, false, false, true, false, false],
                vec![true, false, false, true, false, false],
                vec![false, true, true, false, false, false],
                vec![false, false, false, false, false, false],
            ],
            'P',
        ),
        (
            vec![
                vec![true, true, true, true, true, true],
                vec![true, false, false, true, false, false],
                vec![true, false, false, true, true, false],
                vec![false, true, true, false, false, true],
                vec![false, false, false, false, false, false],
            ],
            'R',
        ),
        (
            vec![
                vec![true, true, true, true, true, false],
                vec![false, false, false, false, false, true],
                vec![false, false, false, false, false, true],
                vec![true, true, true, true, true, false],
                vec![false, false, false, false, false, false],
            ],
            'U',
        ),
        (
            vec![
                vec![true, true, false, false, false, false],
                vec![false, false, true, false, false, false],
                vec![false, false, false, true, true, true],
                vec![false, false, true, false, false, false],
                vec![true, true, false, false, false, false],
            ],
            'Y',
        ),
        (
            vec![
                vec![true, false, false, false, true, true],
                vec![true, false, false, true, false, true],
                vec![true, false, true, false, false, true],
                vec![true, true, false, false, false, true],
                vec![false, false, false, false, false, false],
            ],
            'Z',
        ),
    ]
    .iter()
    .cloned()
    .collect();

    letters
}

pub fn find_letter(flags: Vec<Vec<bool>>) -> char {
    let letters = get_letters();

    letters[&flags]
}
