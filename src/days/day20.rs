use std::{
    collections::{HashMap, VecDeque},
    fmt::format,
};

use itertools::Itertools;

use crate::util;

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let mut modules: HashMap<String, Box<dyn Module>> = util::non_empty_lines(input)
        .map(|s| parse_module(&s))
        .map_ok(|m| (m.id(), m))
        .collect::<Result<HashMap<_, _>>>()?;

    let keys = modules.keys().map(|s| s.to_owned()).collect_vec();

    // Make sure all Conjunction modules know about their inputs.
    for id in keys.iter() {
        let m = modules.get(id).unwrap();

        for d in m.destinations() {
            if let Some(dst) = modules.get_mut(&d) {
                dst.set_input(id);
            } else {
                println!("dst {d} not found!");
            }
        }
    }

    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..1000 {
        // Pulses are always processed in the order they are sent. So, if a
        // pulse is sent to modules a, b, and c, and then module a processes
        // its pulse and sends more pulses, the pulses sent to modules b and c
        // would have to be handled first.
        //
        // This is a breadth first traversal of the module tree, so let's use a
        // fifo queue,

        let mut fifo = VecDeque::new();

        // When you push the button, a single low pulse is sent directly to the
        // broadcaster module.
        fifo.push_back(vec![PulseRequest {
            to: "broadcaster".to_string(),
            from: "button".to_string(),
            pulse: Pulse::Low,
        }]);

        // After pushing the button, you must wait until all pulses have been
        // delivered and fully handled before pushing it again.
        while !fifo.is_empty() {
            let reqs = fifo.pop_front().unwrap();

            for req in reqs {
                match req.pulse {
                    Pulse::High => high_count += 1,
                    Pulse::Low => low_count += 1,
                }

                if let Some(dst) = modules.get_mut(&req.to) {
                    let out = dst.send_pulse(&req.from, &req.pulse);
                    if !out.is_empty() {
                        fifo.push_back(out);
                    }
                }
            }
        }
    }

    Ok((high_count * low_count).to_string())
}

#[test]
fn test_part1() {
    assert_eq!("32000000", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let mut modules: HashMap<String, Box<dyn Module>> = util::non_empty_lines(input)
        .map(|s| parse_module(&s))
        .map_ok(|m| (m.id(), m))
        .collect::<Result<HashMap<_, _>>>()?;

    let keys = modules.keys().map(|s| s.to_owned()).collect_vec();
    let mut rx = Rx {
        id: "rx".to_string(),
        inputs: Vec::new(),
    };

    // Make sure all Conjunction modules know about their inputs.
    for id in keys.iter() {
        let m = modules.get(id).unwrap();

        for d in m.destinations() {
            if d == "rx" {
                rx.set_input(id);
            } else if let Some(dst) = modules.get_mut(&d) {
                dst.set_input(id);
            } else {
                println!("dst {d} not found!");
            }
        }
    }

    // The rx has a single feeder module. We are interested in the behavior of
    // it's inputs. They will each have a certain number of button presses
    // before giving a high input, and the total number of button presses needed
    // can be calculated by finding the LCM of those cycles.
    let rx_feeder = rx.inputs.first().unwrap();
    let mut rx_feeder_inputs: HashMap<String, u64> = modules
        .get(rx_feeder)
        .unwrap()
        .inputs()
        .iter()
        .map(|i| (i.to_owned(), 0))
        .collect();

    'press_button: for i in 1.. {
        // Pulses are always processed in the order they are sent. So, if a
        // pulse is sent to modules a, b, and c, and then module a processes
        // its pulse and sends more pulses, the pulses sent to modules b and c
        // would have to be handled first.
        //
        // This is a breadth first traversal of the module tree, so let's use a
        // fifo queue,

        let mut fifo = VecDeque::new();

        // When you push the button, a single low pulse is sent directly to the
        // broadcaster module.
        fifo.push_back(vec![PulseRequest {
            to: "broadcaster".to_string(),
            from: "button".to_string(),
            pulse: Pulse::Low,
        }]);

        // After pushing the button, you must wait until all pulses have been
        // delivered and fully handled before pushing it again.
        while !fifo.is_empty() {
            let reqs = fifo.pop_front().unwrap();

            for req in reqs {
                if let Some(dst) = modules.get_mut(&req.to) {
                    let out = dst.send_pulse(&req.from, &req.pulse);
                    if !out.is_empty() {
                        fifo.push_back(out);
                    }
                }

                if let Some(&v) = rx_feeder_inputs.get(&req.from) {
                    if v == 0 && req.pulse == Pulse::High {
                        // One of the inputs gave a high output. Remember the
                        // number of button presses required for this to happen.
                        rx_feeder_inputs.insert(req.to.clone(), i);
                    }
                }

                if rx_feeder_inputs.values().all(|&v| v > 0) {
                    // We have found the cycle values for all inputs. We can
                    // stop pressing the button.
                    break 'press_button;
                }
            }
        }
    }

    let count = rx_feeder_inputs
        .values()
        .fold(1, |acc, &v| num::integer::lcm(acc, v));

    Ok(count.to_string())
}

#[cfg(test)]
const TEST_INPUT: &str = r"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

#[derive(Copy, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
}

struct PulseRequest {
    to: String,
    from: String,
    pulse: Pulse,
}

trait Module {
    fn id(&self) -> String;
    fn destinations(&self) -> Vec<String>;
    fn set_input(&mut self, id: &str);
    fn inputs(&self) -> Vec<String>;
    fn send_pulse(&mut self, from: &str, pulse: &Pulse) -> Vec<PulseRequest>;
}

fn parse_module(input: &str) -> Result<Box<dyn Module>> {
    let (name, destinations) = input.split_once(" -> ").unwrap();

    if name == "broadcaster" {
        Ok(Box::new(Broadcast {
            id: name.to_string(),
            inputs: Vec::new(),
            destinations: destinations
                .split(",")
                .map(|s| s.trim().to_string())
                .collect_vec(),
        }))
    } else if name.starts_with("%") {
        Ok(Box::new(FlipFlop {
            on: false,
            id: name[1..].to_string(),
            inputs: Vec::new(),
            destinations: destinations
                .split(",")
                .map(|s| s.trim().to_string())
                .collect_vec(),
        }))
    } else if name.starts_with("&") {
        Ok(Box::new(Conjunction {
            memory: HashMap::new(),
            id: name[1..].to_string(),
            destinations: destinations
                .split(",")
                .map(|s| s.trim().to_string())
                .collect_vec(),
        }))
    } else {
        Err(format!("invalid name: {name}"))
    }
}

struct FlipFlop {
    id: String,
    on: bool,
    inputs: Vec<String>,
    destinations: Vec<String>,
}

impl Module for FlipFlop {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn set_input(&mut self, id: &str) {
        self.inputs.push(id.to_string());
    }

    fn inputs(&self) -> Vec<String> {
        self.inputs.clone()
    }

    fn send_pulse(&mut self, _from: &str, pulse: &Pulse) -> Vec<PulseRequest> {
        let mut requests = Vec::new();
        match pulse {
            Pulse::High => {
                // If a flip-flop module receives a high pulse, it is
                // ignored and nothing happens.
            }
            Pulse::Low => {
                // However, if a flip-flop module receives a low pulse, it
                // flips between on and off.
                if self.on {
                    // If it was on, it turns off and sends a low pulse.
                    self.on = false;
                    for d in self.destinations.iter_mut() {
                        requests.push(PulseRequest {
                            to: d.clone(),
                            from: self.id.clone(),
                            pulse: Pulse::Low,
                        });
                    }
                } else {
                    // If it was off, it turns on and sends a high pulse.
                    self.on = true;
                    for d in self.destinations.iter_mut() {
                        requests.push(PulseRequest {
                            to: d.clone(),
                            from: self.id.clone(),
                            pulse: Pulse::High,
                        });
                    }
                }
            }
        }

        requests
    }
}

struct Conjunction {
    id: String,
    memory: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Module for Conjunction {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn set_input(&mut self, id: &str) {
        self.memory.insert(id.to_string(), Pulse::Low);
    }

    fn inputs(&self) -> Vec<String> {
        self.memory.keys().map(|s| s.to_string()).collect_vec()
    }

    fn send_pulse(&mut self, from: &str, pulse: &Pulse) -> Vec<PulseRequest> {
        let mut requests = Vec::new();

        // When a pulse is received, the conjunction module first
        // updates its memory for that input.
        self.memory.insert(from.to_owned(), *pulse);

        if self.memory.values().all(|&p| p == Pulse::High) {
            // Then, if it remembers high pulses for all inputs, it sends a
            // low pulse.
            for d in self.destinations.iter() {
                requests.push(PulseRequest {
                    to: d.clone(),
                    from: self.id.clone(),
                    pulse: Pulse::Low,
                });
            }
        } else {
            // Otherwise, it sends a high pulse
            for d in self.destinations.iter_mut() {
                requests.push(PulseRequest {
                    to: d.clone(),
                    from: self.id.clone(),
                    pulse: Pulse::High,
                });
            }
        }

        requests
    }
}

struct Broadcast {
    id: String,
    inputs: Vec<String>,
    destinations: Vec<String>,
}

impl Module for Broadcast {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn destinations(&self) -> Vec<String> {
        self.destinations.clone()
    }

    fn set_input(&mut self, id: &str) {
        self.inputs.push(id.to_string());
    }

    fn inputs(&self) -> Vec<String> {
        self.inputs.clone()
    }

    fn send_pulse(&mut self, _from: &str, pulse: &Pulse) -> Vec<PulseRequest> {
        let mut requests = Vec::new();

        // There is a single broadcast module (named broadcaster). When
        // it receives a pulse, it sends the same pulse to all of its
        // destination modules.
        for d in self.destinations.iter() {
            requests.push(PulseRequest {
                to: d.clone(),
                from: self.id.clone(),
                pulse: *pulse,
            });
        }

        requests
    }
}

struct Rx {
    id: String,
    inputs: Vec<String>,
}

impl Module for Rx {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn destinations(&self) -> Vec<String> {
        Vec::new()
    }

    fn set_input(&mut self, id: &str) {
        self.inputs.push(id.to_string());
    }

    fn inputs(&self) -> Vec<String> {
        self.inputs.clone()
    }

    fn send_pulse(&mut self, _from: &str, pulse: &Pulse) -> Vec<PulseRequest> {
        Vec::new()
    }
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
