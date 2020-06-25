use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Opt {
    /// [W]
    microwave_wattage: f64,

    /// [s]
    heating_time: f64,

    /// [g]
    water_weight: f64,

    /// [℃]
    temperature_before_heating: f64,

    /// [℃]
    temperature_after_heating: f64,
}

fn efficiency(opt: &Opt) -> f64 {
    const WATER_SPECIFIC_HEAT: f64 = 4.2;
    let temperature_diff = opt.temperature_after_heating - opt.temperature_before_heating;
    let j = WATER_SPECIFIC_HEAT * opt.water_weight * temperature_diff;
    let w = j / opt.heating_time;
    w / opt.microwave_wattage
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", efficiency(&opt));
}
