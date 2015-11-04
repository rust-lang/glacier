use nom::{IResult, not_line_ending, line_ending};

fn csv_line(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    terminated!(input, separated_list!(filter!(apply!(check_characters, b"\n\r,")), not_line_ending), line_ending)
}

fn check_characters(data: &[u8], characters: &[u8]) -> bool {
    for i in 0..data.len() {
        for c in characters {
            if data[i] == *c {
                return false;
            }
        }
    }
    true
}

#[test]
fn check_file() {
    let f = b"nom,age\ncarles,30\nlaure,28";

    csv_line(f);
}
