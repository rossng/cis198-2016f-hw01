pub fn hanoi(num_discs: u32) -> Vec<(u8, u8)> {
    let mut solution: Vec<(u8, u8)> = vec![];

    move_discs(num_discs, 1, 3, 2, &mut solution);

    solution
}

fn move_discs(num_discs: u32, source: u8, target: u8, auxiliary: u8, solution: &mut Vec<(u8, u8)>) {
    if num_discs > 0 {
        move_discs(num_discs-1, source, auxiliary, target, solution);

        solution.push((source, target));

        move_discs(num_discs-1, auxiliary, target, source, solution);
    }
}
