use std::fs;

#[derive(Debug)]
struct Graph {
    nodes: Vec<String>,
    target_nodes: Vec<i64>,
    flow_rates: Vec<i64>,
    edges: Vec<Vec<usize>>,
    pub target_mask: i64,
    cache: Vec<i64>,
    node_count: i64,
}

impl Graph {
    fn new(nodes: Vec<String>, flow_rates: Vec<i64>, edges: Vec<(String, String)>) -> Graph {
        let mut target_nodes = vec![-1; nodes.len()];
        let mut target_node_index = 0;
        for (i, _) in flow_rates.iter().enumerate().filter(|(_, &r)| r > 0) {
            target_nodes[i] = target_node_index;
            target_node_index += 1;
        }
        let target_mask = (1 << target_node_index) - 1;
        let node_count = nodes.len();
        let mut g = Graph {
            nodes,
            target_nodes,
            flow_rates,
            edges: vec![vec![]; node_count],
            target_mask,
            cache: vec![-1; target_mask as usize * node_count * 31],
            node_count: node_count as i64,
        };
        for (src, dst) in edges {
            let si = g.node_index(src);
            let di = g.node_index(dst);
            g.edges.get_mut(si).unwrap().push(di);
        }
        g
    }

    fn node_index(&self, label: String) -> usize {
        self.nodes.iter().position(|t| t.eq(&label)).unwrap()
    }

    fn max_pressure(&mut self, i: usize, t: i64, visited: i64) -> i64 {
        if visited == self.target_mask || t == 0 {
            return 0;
        }
        let k = i + (visited * self.node_count * 31 + t * self.node_count) as usize;
        if self.cache[k] >= 0 {
            return self.cache[k];
        }

        let mut max = 0;
        if self.target_nodes[i] >= 0 && visited & (1 << self.target_nodes[i]) == 0 {
            max = self.flow_rates[i] * (t - 1)
                + self.max_pressure(i, t - 1, visited | (1 << self.target_nodes[i]));
        }
        for &e in self.edges[i].clone().iter() {
            max = max.max(self.max_pressure(e, t - 1, visited));
        }
        self.cache[k] = max;
        max
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/16.txt").unwrap();

    let mut nodes = vec![];
    let mut flow_rates = vec![];
    let mut edges = vec![];

    for l in input.trim().split('\n') {
        let p = l
            .split(|c| [' ', '=', ';'].contains(&c))
            .collect::<Vec<_>>();
        nodes.push(p[1].to_string());
        flow_rates.push(p[5].parse::<i64>().unwrap());
    }
    for l in input.trim().split('\n') {
        let p = l.split(|c| [' ', ','].contains(&c)).collect::<Vec<_>>();
        let src = p[1].to_string();
        for i in (9..p.len()).step_by(2) {
            edges.push((src.clone(), p[i].to_string()));
        }
    }

    let mut g = Graph::new(nodes, flow_rates, edges);
    let start = g.node_index("AA".to_string());

    println!("part1: {:#?}", g.max_pressure(start, 30, 0));

    let mut part2_max = 0;
    for m in 0..g.target_mask {
        part2_max = part2_max
            .max(g.max_pressure(start, 26, m) + g.max_pressure(start, 26, g.target_mask ^ m));
    }
    println!("part2: {:#?}", part2_max);
}
