use crate::petri_net_model::*;

const vertices_count: usize = 45;
const cites_count: usize = 7;
const vertices_per_city: usize = 3;
const trains_id: usize = 0;
const source_id: usize = 1;
const vertices_per_t: usize = 3;
const train_state: usize = 1;
const train_with_source_state: usize = 2;

pub fn fast_create_small_road(from: usize, to: usize) -> Vec<PetriNetTransition> {
    let first = from * vertices_per_city;
    let second = to * vertices_per_city;
    let mut t1 = vec![0; vertices_count];
    t1[first] = -1;
    t1[first + 1] = -1;
    t1[first + 2] = 1;
    t1[second] = 1;
    t1[second + 1] = 1;
    t1[second + 2] = -1;
    let mut t2 = vec![0; vertices_count];
    t2[first] = -1;
    t2[second] = 1;
    vec![PetriNetTransition::new(t1), PetriNetTransition::new(t2)]
}

pub fn fast_create_road(from: usize, to: usize, ind: usize) -> Vec<PetriNetTransition> {
    let first = from * vertices_per_city;
    let second = to * vertices_per_city;
    let addr = vertices_per_city * cites_count + vertices_per_t * ind; 

    let mut t1 = vec![0; vertices_count];
    t1[first] = -1;
    t1[first + 1] = -1;
    if from > 1 {
        t1[first + 2] = 1;
    }
    t1[addr] = -1;
    t1[addr + 2] = 1;
    if to > 1 {
        t1[second + 2] = -1;
    }

    let mut t2 = vec![0; vertices_count];
    t2[second] = 1;
    t2[second + 1] = 1;
    t2[addr] = 1;
    t2[addr + 2] = -1;

    let mut t3 = vec![0; vertices_count];
    t3[first] = -1;
    t3[addr] = -1;
    t3[addr + 1] = 1;

    let mut t4 = vec![0; vertices_count];
    t4[second] = 1;
    t4[addr] = 1;
    t4[addr + 1] = -1;

    vec![t1, t2, t3, t4].into_iter().map(PetriNetTransition::new).collect()
}

pub fn fast_create_big_road(from: usize, to: usize, ind: usize) -> Vec<PetriNetTransition> {
    let first = from * vertices_per_city;
    let second = to * vertices_per_city;
    let addr = vertices_per_city * cites_count + vertices_per_t * ind; 

    let mut t1 = vec![0; vertices_count];
    t1[first] = -1;
    t1[first + 1] = -1;
    if from > 1 {
        t1[first + 2 ] = 1;
    }
    t1[addr] = -1;
    t1[addr + 2] = 1;
    if to > 1 {
        t1[second + 2] = -1;
    }
    let mut t2 = vec![0; vertices_count];
    t2[addr] = 1;
    t2[addr + 2] = -1;
    t2[addr + 3] = -1;
    t2[addr + 5] = 1;
    let mut t3 = vec![0; vertices_count];
    t3[addr + 5] = -1;
    t3[addr + 3] = 1;
    t3[second] = 1;
    t3[second + 1] = 1;
    let mut t4 = vec![0; vertices_count];
    t4[first] = -1;
    t4[addr] = -1;
    t4[addr + 1] = 1;
    let mut t5 = vec![0; vertices_count];
    t5[addr] = 1;
    t5[addr + 1] = -1;
    t5[addr + 3] = -1;
    t5[addr + 4] = 1;
    let mut t6 = vec![0; vertices_count];
    t6[addr + 3] = 1;
    t6[addr + 4] = -1;
    t6[second] = 1;

    vec![t1, t2, t3, t4, t5, t6].into_iter().map(PetriNetTransition::new).collect()
}

pub fn transitions() -> Vec<PetriNetTransition> {
    let t_0_2 = fast_create_big_road(0, 2, 0);// 0 - 5
    let t_1_0 = fast_create_road(1, 0, 2); // 6 - 9
    let t_1_5 = fast_create_road(1, 5, 3); // 10 - 13
    let t_2_3 = fast_create_road(2, 3, 4); // 14 - 18
    let t_3_1 = fast_create_big_road(3, 1, 5); // 18 - 21
    let t_4_0 = fast_create_road(4, 0, 7); // 22 - 27
    let t_4_5 = fast_create_small_road(4, 5); // 28 - 29
    let t_5_6 = fast_create_small_road(5, 6); // 30 - 31
    let t_6_4 = fast_create_small_road(6, 4); // 32 - 33

    vec![t_0_2, t_1_0, t_1_5, t_2_3, t_3_1, t_4_0, t_4_5, t_5_6, t_6_4].concat()
}

pub fn our_model() -> PetriNet {
    PetriNet::new(vertices_count, transitions())
}

pub fn reverse_transmition_by_cycle(t: PetriNetTransition, mut c: Vec<PetriNetTransition>) -> Option<Vec<PetriNetTransition>> {
    let index = c.iter().position(|x| x == &t)? + 1;
    c.rotate_left(index);
    c.pop();
    Some(c)
}

pub fn reverse_transmition(t: PetriNetTransition) -> Vec<PetriNetTransition> {
    todo!()
}

pub fn transitions_example() -> Vec<Vec<PetriNetTransition>> {
    transitions()
        .into_iter()
        .map(reverse_transmition)
        .collect()
}

pub fn paint_example() -> Vec<Color> {
    (0..vertices_count)
        .map(|x| {
            if x < cites_count * vertices_per_city {
                if x % vertices_per_city == trains_id {
                    Color::Red
                } else if x % vertices_per_city == source_id {
                    Color::Blue
                } else {
                    Color::None
                }
            } else {
                if x % vertices_per_t == train_state {
                    Color::Red
                } else if x % vertices_per_t == train_with_source_state {
                    Color::Purple
                } else {
                    Color::None
                }
            }
        })
        .collect()
        
}

#[cfg(test)]
mod tests {
    use super::{our_model, paint_example};
    #[test]
    fn color_test() {
        println!("{}", our_model().check_duality(&paint_example()));
        assert!(our_model().check_duality(&paint_example()))
    }
}