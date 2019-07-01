
pub fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {

    let mut lands = grid;

    (0..lands.len()).fold(0, |mut islas, i| {
        islas += (0..lands[0].len()).fold(0, |isla, j| {
            if lands[i][j] == '1' {
                recursiva_searching_sorrounded_lands_2(&mut lands, i, j);
                isla + 1
            } else {
                isla
            }
        });
        islas
    })

}

fn recursiva_searching_sorrounded_lands_2(lands: &mut Vec<Vec<char>>, i: usize, j: usize) {

    lands[i][j] = '0';

    if j + 1 < lands[0].len() && lands[i][j + 1] == '1' {
        recursiva_searching_sorrounded_lands_2(lands, i, j + 1);
    }
    if i + 1 < lands.len() && lands[i + 1][j] == '1' {
        recursiva_searching_sorrounded_lands_2(lands, i + 1, j);
    }

    if j > 0 && lands[i][j - 1] == '1' {
        recursiva_searching_sorrounded_lands_2(lands, i, j - 1);
    }

    if i > 0 && lands[i - 1][j] == '1' {
        recursiva_searching_sorrounded_lands_2(lands, i - 1, j);
    }
}

