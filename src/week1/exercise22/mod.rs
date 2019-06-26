struct FriendCircle {
    index_vector: Vec<i32>,
    circle_count: i32,
}

impl FriendCircle {
    fn new(matrix_order: i32) -> FriendCircle {
        FriendCircle {
            index_vector: (0..matrix_order).map(|x| x).collect(),
            circle_count: matrix_order,
        }
    }

    fn find(&mut self, mut position: usize) -> usize {
        let friend = self.index_vector[position] as usize;
        if position != friend {
            self.index_vector[position] = self.index_vector[position];
            position = friend;
        }
        position
    }
}

pub fn count_friend_circle(input: Vec<Vec<i32>>) -> i32 {
    let matrix_order = input.len() as i32;
    let mut friend_circle = FriendCircle::new(matrix_order);

    input.into_iter().enumerate().for_each(|(i, row)| {
        get_one_count(i, row, &mut friend_circle);
    });

    friend_circle.circle_count
}

fn get_one_count(i: usize, row: Vec<i32>, friend_circle: &mut FriendCircle) {
    row.into_iter().enumerate().for_each(|(j, element)| {
        println!("element {}", element);

        let a = friend_circle.find(i);
        let b = friend_circle.find(j);

        if element == 1 && a != b {
            friend_circle.index_vector[a] = b as i32;
            friend_circle.circle_count -= 1;
        }
    });
}
