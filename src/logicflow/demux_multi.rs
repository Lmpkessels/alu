/* 
4-Way Demuxer, returns an array of 4 arrays, with only one active slot.

The selector (`sel: u8`) receives a number from 0..=3, based on that,
an active slot is selected, all other slots remain zeroed.

Else, an error message is raised.

Input (inp: [u8; 8]) == declared as the active slot.
*/
fn dmux4way(inp: &[u8; 8], sel: u8) -> [[u8; 8]; 4] {
    // If selector == 0 first slot is active.
    if sel == 0 {
        let array = [
            *inp,
            [0; 8], 
            [0; 8], 
            [0; 8],
        ];
        println!("{array:?}");
        return array;
    }
    // If selector == 1 second slot is active.
    if sel == 1 {
        let array = [
            [0; 8],
            *inp, 
            [0; 8],
            [0; 8],
        ];
        println!("{array:?}");
        return array;
    }
    // If selector == 2 third slot is active.
    if sel == 2 {
        let array = [
            [0; 8], 
            [0; 8], 
            *inp, 
            [0; 8],
        ];
        println!("{array:?}");
        return array;
    }
    // If selector == 3 forth slot is active.
    if sel == 3 {
        let array = [
            [0; 8], 
            [0; 8], 
            [0; 8], 
            *inp,
        ];
        println!("{array:?}");
        return array;
    }

    // Handle invalid selector input.
    panic!("Invalid! selector: must be 0-3");
}

fn main() {
    const IN: [u8; 8] = [1; 8];
    dmux4way(&IN, 1);
}