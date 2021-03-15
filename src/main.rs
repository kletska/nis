mod petri_net_model;
mod our_model;

fn main() {
    println!("Hello, world!");
}
/* 
struct Graph {
    vertices: HashMap<PetriNetStage, u32>,
    adges: Vec<Vec<u32>>, 
}

impl Graph {
    pub fn new(net: &PetriNet) -> Graph {
        let mut cnt = 0;
        let mut used: HashMap<PetriNetStage, u32> = HashMap::new();
        let mut queue: VecDeque<PetriNetStage> = VecDeque::new();

        queue.push_back(net.start.clone());
        used.insert(net.start.clone(), cnt);
        cnt += 1;

        while let Some(front) = queue.pop_front() {
            for i in net.possible_moves(&front) {
                if !used.contains_key(&i) {
                    queue.push_back(i.clone());
                    used.insert(i, cnt);
                    cnt += 1;
                }
            }
        }

        todo!()
    }
}
*/