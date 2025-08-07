/* 
4-Way Demuxer, returns an array of 4 arrays, with only one active slot.

The selector (`sel: u8`) receives a number from 0..=3, based on that,
an active slot is selected, all other slots remain zeroed.

If outside of range, an error message is raised.

Input (inp: [u8; 8]) == declared as the active slot.
*/
fn dmux4way(active_slot: [u8; 32], selector: u8) -> [[u8; 32]; 4] {

    if selector == 0 {
        let array = [
            active_slot,
            [0; 32], 
            [0; 32], 
            [0; 32],
        ];
        println!("{array:?}");
        return array;
    }

    if selector == 1 {
        let array = [
            [0; 32],
            active_slot, 
            [0; 32],
            [0; 32],
        ];
        println!("{array:?}");
        return array;
    }
    
    if selector == 2 {
        let array = [
            [0; 32], 
            [0; 32], 
            active_slot, 
            [0; 32],
        ];
        println!("{array:?}");
        return array;
    }

    if selector == 3 {
        let array = [
            [0; 32], 
            [0; 32], 
            [0; 32], 
            active_slot,
        ];
        println!("{array:?}");
        return array;
    }

    panic!("Invalid! selector: must be 0-3");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn if_sel_is_0_then_it_should_return_only_the_first_slot_active() {
        let active_slot = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1];
        let selector = 0;
        let expected = [
            active_slot,
            [0; 32],
            [0; 32],
            [0; 32],
        ];

        assert_eq!(dmux4way(active_slot, selector), (expected));
    }

    #[test]
    fn if_sel_is_1_then_it_should_return_the_second_slot_active() {
        let active_slot = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1];
        let selector = 1;
        let expected = [
            [0; 32],
            active_slot,
            [0; 32],
            [0; 32],
        ];

        assert_eq!(dmux4way(active_slot, selector), (expected));
    }

    #[test]
    fn if_sel_is_2_then_it_should_return_the_third_slot_active() {
        let active_slot = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1];
        let selector = 2;
        let expected = [
            [0; 32],
            [0; 32],
            active_slot,
            [0; 32],
        ];

        assert_eq!(dmux4way(active_slot, selector), (expected));
    }

    #[test]
    fn if_sel_is_3_then_it_should_return_the_last_slot_active() {
        let active_slot = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1];

        let selector = 3;
        let expected = [
            [0; 32],
            [0; 32],
            [0; 32],
            active_slot,
        ];

        assert_eq!(dmux4way(active_slot, selector), (expected));
    }
}