use std::fs;
use std::result::Result;

pub struct CPUInfo {
    pub model: String,
    pub cores: usize,
    pub freq:  usize,
}

impl CPUInfo {
    pub fn new() -> CPUInfo {
        CPUInfo {
            model: String::new(),
            cores: 0,
            freq:  0,
        }
    }

    // retrieve model, cores, and frequency
    pub fn get(&mut self) -> Result<(), std::io::Error> {
        // model and number of cores
        if !fs::metadata("/proc/cpuinfo").is_ok() {
            return Err(());
        }

        let cpuinfos = fs::read_to_string("/proc/cpuinfo")?;
        for line in cpuinfos.split("\n") {
            let cpuinfo = line.split(":").map(|i| i.trim()).collect::<Vec<&str>>();
            match cpuinfo[0] {
                "Hardware" => self.model = cpuinfo[1].to_string(),
                "processor" => self.cores = cpuinfo[1].parse::<usize>()? + 1,
                _ => (),
            }
        }

        // frequency
        if !fs::metadata("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq").is_ok() {
            return Err(());
        }

        self.freq = (fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")?
            .parse::<usize>()?) / 1000;

        Ok(())
    }
}
