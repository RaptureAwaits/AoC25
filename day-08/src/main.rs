use std::cmp::Ordering;
use aoc_shared::{get_input_filepath, line_iterator};

type CircuitIndex = usize;
type JunctionIndex = usize;

struct Distance {
    from: CircuitIndex,
    to: CircuitIndex,
    dist: f64
}

impl Distance {
    fn new(from: CircuitIndex, to: CircuitIndex, dist: f64) -> Distance {
        Distance { from, to, dist }
    }
}

impl Eq for Distance {}

impl PartialEq<Self> for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd<Self> for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.total_cmp(&other.dist)
    }
}

struct CircuitNetwork {
    circuits: Vec<Circuit>,
}

impl CircuitNetwork {
    fn new() -> CircuitNetwork {
        CircuitNetwork { circuits: vec![] }
    }

    fn new_circuit(&mut self) -> CircuitIndex {
        let index = self.circuits.len();
        self.circuits.push(Circuit::new(index));
        index
    }

    fn get_circuit(&mut self, index: CircuitIndex) -> &mut Circuit {
        &mut self.circuits[index]
    }
}

struct JunctionNetwork {
    junctions: Vec<Junction>,
}

impl JunctionNetwork {
    fn new() -> JunctionNetwork {
        JunctionNetwork { junctions: vec![] }
    }

    fn new_junction(&mut self, x: i64, y: i64, z: i64) -> JunctionIndex {
        let index = self.junctions.len();
        self.junctions.push(Junction::new(x, y, z, index));
        index
    }

    fn get_junction(&self, index: JunctionIndex) -> &Junction {
        &self.junctions[index]
    }

    fn get_junction_mut(&mut self, index: JunctionIndex) -> &mut Junction {
        &mut self.junctions[index]
    }
}

struct Circuit {
    index: CircuitIndex,
    junctions: Vec<JunctionIndex>,
}

impl Circuit {
    fn new(index: CircuitIndex) -> Circuit {
        Circuit { index, junctions: vec![] }
    }
}

struct Junction {
    x: i64,
    y: i64,
    z: i64,

    index: JunctionIndex,
    circuit: Option<CircuitIndex>
}

impl Junction {
    fn new(x: i64, y: i64, z: i64, j_index: JunctionIndex) -> Junction {
        Junction { x, y, z, index: j_index, circuit: None }
    }

    fn calc_distance(&self, other: &Junction) -> Distance {
        let dist = {
            {
                self.x - other.x
            }.pow(2) as f64 + {
                self.y - other.y
            }.pow(2) as f64 + {
                self.z - other.z
            }.pow(2) as f64
        }.sqrt();

        Distance::new(self.index, other.index, dist)
    }
}

fn main() {
    let filepath = get_input_filepath();

    if let Ok(rows) = line_iterator(&filepath) {
        let mut junction_network = JunctionNetwork::new();
        let mut circuit_network = CircuitNetwork::new();
        let mut distances: Vec<Distance> = vec![];

        // Process junctions from input, calculate distances in network
        for row in rows.map_while(Result::ok) {
            let coords = row.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let (x, y, z) = match &coords[..] {
                &[x,y,z] => (x, y, z),
                _ => panic!("Invalid co-ordinates given")
            };

            let new_index = junction_network.new_junction(x, y, z);
            for junction in junction_network.junctions[..new_index].iter() {
                let new_junction = junction_network.get_junction(new_index);
                distances.push(new_junction.calc_distance(junction));
            }
        }

        distances.sort();

        let mut distance_index: usize = 10;
        for distance in distances[..distance_index].iter() {
            connect_junctions(distance, &mut junction_network, &mut circuit_network);
        }

        let mut populated_circuits = circuit_network.circuits.iter().filter(|c| c.junctions.len() > 0).collect::<Vec<&Circuit>>();
        populated_circuits.sort_by(|c1, c2| c1.junctions.len().cmp(&c2.junctions.len()));
        populated_circuits.reverse();

        let mut size_mult = 1;
        for circuit in populated_circuits[..3].iter() {
            size_mult *= circuit.junctions.len();
        }
        println!("The multiplied size of the largest 3 circuits is {}", size_mult);

        while circuit_network.circuits.iter().map(|c| c.junctions.len()).max() < Some(junction_network.junctions.len()) {
            connect_junctions(&distances[distance_index], &mut junction_network, &mut circuit_network);
            distance_index += 1;
        }

        let x1 = junction_network.get_junction(distances[distance_index - 1].from).x;
        let x2 = junction_network.get_junction(distances[distance_index - 1].to).x;
        println!("The total of the x co-ordinates of the last 2 connected junctions is {} * {} = {}", x1, x2, x1 * x2);
    }
}

fn connect_junctions(distance: &Distance, junction_network: &mut JunctionNetwork, circuit_network: &mut CircuitNetwork) {
    let j1_circuit_index = junction_network.get_junction(distance.from).circuit;
    let j2_circuit_index = junction_network.get_junction(distance.to).circuit;

    {
        let j1 = junction_network.get_junction(distance.from);
        let j2 = junction_network.get_junction(distance.to);
        // println!(
        //     "Connecting {} ({}, {}, {}) to {} ({}, {}, {}) - dist = {}",
        //     j1.index, j1.x, j1.y, j1.z,
        //     j2.index, j2.x, j2.y, j2.z,
        //     distance.dist
        // );
    }

    match (j1_circuit_index, j2_circuit_index) {
        (Some(c1_index), Some(c2_index)) => {  // Merge two circuits (c1, c2) into one (c2)
            if c1_index == c2_index {
                return
            }

            let c1_junctions = circuit_network.get_circuit(c1_index).junctions
                .drain(..)
                .collect::<Vec<JunctionIndex>>();

            let c2 = circuit_network.get_circuit(c2_index);
            for j_index in c1_junctions {
                let j = junction_network.get_junction_mut(j_index);
                j.circuit = Some(c2_index);  // Update membership field of junction
                c2.junctions.push(j_index);  // Update ownership list of circuit
            }
        },

        (Some(c1_index), None) => {  // Add j2 to c1
            let c1 = circuit_network.get_circuit(c1_index);
            let j2 = junction_network.get_junction_mut(distance.to);
            j2.circuit = Some(c1_index);
            c1.junctions.push(j2.index);
        },

        (None, Some(c2_index)) => {  // Add j1 to c2
            let c2 = circuit_network.get_circuit(c2_index);
            let j1 = junction_network.get_junction_mut(distance.from);
            j1.circuit = Some(c2_index);
            c2.junctions.push(j1.index);
        },

        (None, None) => {  // Create a new circuit with j1 and j2
            let c_new_index = circuit_network.new_circuit();
            let c_new = circuit_network.get_circuit(c_new_index);

            {
                let j1 = junction_network.get_junction_mut(distance.from);
                j1.circuit = Some(c_new.index);
                c_new.junctions.push(j1.index);
            }

            {
                let j2 = junction_network.get_junction_mut(distance.to);
                j2.circuit = Some(c_new.index);
                c_new.junctions.push(j2.index);
            }
        }
    }
    // println!("{}", circuit_network.circuits.iter().map(|c| c.junctions.len().to_string() + ", ").collect::<String>());
}