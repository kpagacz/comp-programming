// Runtime34 ms
// Beats
// 92.86%
// Memory17.2 MB
// Beats
// 85.71%

use std::collections::HashMap;

struct UndergroundSystem {
    check_ins: HashMap<i32, CheckIn>,
    times: HashMap<Connection, Average>,
    stations: HashMap<String, usize>,
}

#[derive(Clone)]
struct CheckIn {
    pub time: f64,
    pub station: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct Connection {
    pub source: usize,
    pub destination: usize,
}

struct Average {
    pub sum: f64,
    pub count: f64,
    pub mem: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            check_ins: HashMap::new(),
            times: HashMap::new(),
            stations: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        let length = self.stations.len();
        let station_id = self.stations.entry(station_name).or_insert(length);
        let check_in = self.check_ins.entry(id).or_insert(CheckIn {
            time: t as f64,
            station: *station_id,
        });
        check_in.time = t as f64;
        check_in.station = *station_id;
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let length = self.stations.len();
        let check_in = self.check_ins.get(&id).unwrap();
        let line_average = self
            .times
            .entry(Connection {
                source: check_in.station,
                destination: *self.stations.entry(station_name).or_insert(length),
            })
            .or_insert(Average {
                sum: 0 as f64,
                count: 0 as f64,
                mem: -1 as f64,
            });
        line_average.sum += t as f64 - check_in.time;
        line_average.count += 1 as f64;
        line_average.mem = -1 as f64;
    }

    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let line_average = self
            .times
            .entry(Connection {
                source: *self.stations.get(&start_station).unwrap(),
                destination: *self.stations.get(&end_station).unwrap(),
            })
            .or_insert(Average {
                sum: 0 as f64,
                count: 0 as f64,
                mem: 0 as f64,
            });
        if line_average.mem != -1 as f64 {
            return line_average.mem;
        }
        line_average.mem = line_average.sum / line_average.count;
        line_average.mem
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

fn main() {
    println!("Hello, world!");
}
