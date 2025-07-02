fn or8multi(inp: [[u8; 8]; 8]) {

    let mut storage: Vec<u8> = Vec::new();

    // Get index 0 to 7 in each array + apply OR logic, manually.
    for i in 0..1 {
        let i_0 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
        inp[6][i] | inp[7][i];
        for i in 1..2 {
            let i_1 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
            inp[6][i] | inp[7][i];
            for i in 2..3 {
                let i_2 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
                inp[6][i] | inp[7][i];
                for i in 3..4 {
                    let i_3 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
                    inp[6][i] | inp[7][i];
                    for i in 4..5 {
                        let i_4 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
                        inp[6][i] | inp[7][i];
                        for i in 5..6 {
                            let i_5 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
                            inp[6][i] | inp[7][i];
                            for i in 6..7 {
                                let i_6 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
                                inp[6][i] | inp[7][i];
                                for i in 7..8 {
                                    let i_7 = inp[0][i] | inp[1][i] | inp[2][i] | inp[3][i] | inp[4][i] | inp[5][i] |
                                    inp[6][i] | inp[7][i];
                                    // Vector storage to push each end 
                                    // result after or, and return it as end result.
                                    let mut storage = Vec::new();
                                    storage.push(i_0);
                                    storage.push(i_1);
                                    storage.push(i_2);
                                    storage.push(i_3);
                                    storage.push(i_4);
                                    storage.push(i_5);
                                    storage.push(i_6);
                                    storage.push(i_7);

                                    println!("{storage:?}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    // Check if everything works.
    let test = [
        [1, 0, 1, 0, 0, 1, 0, 1], 
        [1, 0, 0, 0, 1, 0, 1, 1], 
        [1, 0, 1, 0, 1, 0, 1, 0],
        [1, 1, 1, 0, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 0, 1, 1], 
        [0, 0, 1, 0, 0, 1, 0, 0], 
        [0, 1, 0, 1, 0, 1, 0, 1], 
        [1, 0, 1, 0, 1, 0, 1, 0]
        ];
    
    or8multi(test);
}