use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Debug)]
struct FlipFlop {
    state: Pulse,
}

impl FlipFlop {
    fn new() -> Self {
        Self { state: Pulse::Low }
    }

    fn pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match (self.state, pulse) {
            (Pulse::Low, Pulse::Low) => {
                self.state = Pulse::High;
                Some(Pulse::High)
            }
            (Pulse::High, Pulse::Low) => {
                self.state = Pulse::Low;
                Some(Pulse::Low)
            }
            (_, Pulse::High) => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
    first_high: HashMap<String, u64>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            first_high: HashMap::new(),
        }
    }

    fn pulse(&mut self, button_presses: u64, source: String, pulse: Pulse) -> Pulse {
        self.inputs.insert(source.clone(), pulse);
        if pulse == Pulse::High && self.first_high.get(&source) == None {
            self.first_high.insert(source, button_presses);
        }
        if self.inputs.clone().iter().all(|x| *x.1 == Pulse::High) {
            return Pulse::Low;
        }
        Pulse::High
    }
}

#[derive(Clone, Debug)]
enum SwitchType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster,
}

#[derive(Clone, Debug)]
struct Switch {
    switch_type: SwitchType,
    outputs: Vec<String>,
}

struct SwitchArray {
    button_presses: u64,
    rx_low: Option<u64>,
    switches: HashMap<String, Switch>,
    high_pulses: Option<i32>,
    low_pulses: Option<i32>,
}

impl Switch {
    fn from(pattern: (&str, &str)) -> (String, Self) {
        let switch_type;
        let id;
        if pattern.0.len() > 5 {
            switch_type = SwitchType::Broadcaster;
            id = pattern.0.to_string();
        } else {
            let mut token = pattern.0.chars();
            if token.nth(0) == Some('%') {
                switch_type = SwitchType::FlipFlop(FlipFlop::new());
            } else {
                switch_type = SwitchType::Conjunction(Conjunction::new());
            }
            id = token.collect();
        }

        let outputs = pattern.1.split(", ").map(|x| x.to_string()).collect();

        return (
            id,
            Self {
                switch_type,
                outputs,
            },
        );
    }
}

impl SwitchArray {
    fn new() -> Self {
        Self {
            button_presses: 0,
            rx_low: None,
            switches: HashMap::new(),
            high_pulses: None,
            low_pulses: None,
        }
    }

    fn pulse(&mut self) {
        self.button_presses += 1;
        let mut high_pulses = 0;
        let mut low_pulses = 1;
        let mut queue = VecDeque::new();
        for out in self.switches.get("broadcaster").unwrap().outputs.clone() {
            queue.push_back((String::from("broadcaster"), out, Pulse::Low));
        }
        while queue.len() > 0 {
            let (source, receiver, pulse) = queue.pop_front().unwrap();
            match pulse {
                Pulse::High => {
                    high_pulses += 1;
                }
                Pulse::Low => {
                    low_pulses += 1;
                }
            }

            // hj is the only node sending pulse to rx
            // when all of hj's inputs are high then rx receives a low signal
            if receiver == "hj" && pulse == Pulse::High {
                println!(
                    "press: {}; {} -{:?}-> {}",
                    self.button_presses, source, pulse, receiver
                );
            }

            let switch_or = self.switches.get_mut(&receiver);
            if let Some(switch) = switch_or {
                match switch.switch_type {
                    SwitchType::FlipFlop(ref mut flop) => {
                        if let Some(pulse_out) = flop.pulse(pulse) {
                            for output in switch.outputs.clone() {
                                queue.push_back((receiver.clone(), output, pulse_out));
                            }
                        }
                    }
                    SwitchType::Conjunction(ref mut conj) => {
                        let pulse_out = conj.pulse(self.button_presses, source, pulse);
                        for output in switch.outputs.clone() {
                            queue.push_back((receiver.clone(), output, pulse_out));
                        }
                        // hj is the only node sending pulse to rx
                        // when all of hj's inputs are high then rx receives a low signal
                        if receiver == "hj" && conj.inputs.len() == conj.first_high.len() {
                            self.rx_low =
                                Some(conj.first_high.iter().fold(1, |acc, x| lcm(acc, *x.1)))
                        }
                    }
                    _ => {}
                }
            } else {
                if receiver == String::from("rx") {
                    match (pulse, self.rx_low) {
                        (Pulse::Low, None) => {
                            self.rx_low = Some(self.button_presses);
                        }
                        (_, Some(_)) | (Pulse::High, _) => {}
                    }
                }
            }
        }
        self.high_pulses = Some(high_pulses);
        self.low_pulses = Some(low_pulses);
    }

    fn register_switches(&mut self, switches: Vec<String>) {
        let mut conjunctions = vec![];
        let mut other_switches = vec![];
        for l in switches {
            if l.chars().nth(0) == Some('&') {
                conjunctions.push(l);
            } else {
                other_switches.push(l);
            }
        }
        for switch in conjunctions.iter().chain(other_switches.iter()) {
            self.register_switch(switch);
        }
        self.register_inputs();
    }

    fn register_switch(&mut self, switch: &String) {
        let split = switch.split_once(" -> ");
        if let Some(pattern) = split {
            let (id, switch) = Switch::from(pattern);
            self.switches.insert(id, switch);
        } else {
            panic!("attempted to register invalid sequence!");
        }
    }

    fn register_inputs(&mut self) {
        let mut inputs = vec![];
        for switch in self.switches.clone() {
            for output in switch.1.outputs {
                inputs.push((switch.0.clone(), output));
            }
        }

        for (source, target) in inputs {
            if let Some(switch) = self.switches.get_mut(&target) {
                match switch.switch_type {
                    SwitchType::Conjunction(ref mut conj) => {
                        conj.inputs.insert(source, Pulse::Low);
                    }
                    _ => {}
                };
            }
        }
    }
}

pub fn pulse(path: &Path) -> (Option<i32>, Option<u64>) {
    let file = File::open(path).expect("Error opening file!");
    let reader = BufReader::new(file);

    let mut switch_array = SwitchArray::new();
    switch_array.register_switches(reader.lines().flatten().collect());

    let mut total_low = 0;
    let mut total_high = 0;
    let mut pt2 = None;
    for _ in 0..1000 {
        switch_array.pulse();
        total_low += switch_array.low_pulses.unwrap();
        total_high += switch_array.high_pulses.unwrap();
    }

    for _ in 0..9000 {
        switch_array.pulse();
        if let Some(rx_low) = switch_array.rx_low {
            pt2 = Some(rx_low);
            break;
        }
    }

    return (Some(total_high * total_low), pt2);
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

use std::cmp::min;
use std::mem::swap;

pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}
