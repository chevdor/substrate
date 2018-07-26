
use clap::ArgMatches;

pub struct Advisor {
	advices: Option<Vec<String>>,
}

impl Advisor {
	pub fn new(matches: ArgMatches) -> Advisor {
		println!("agrs {:?}", matches);

		let mut advices = Vec::new();

		if matches.is_present("validator") && !matches.is_present("key") {
			advices.push(String::from("When using --validator, make sure to also pass --key 0x..."));
		}

		Advisor {
			advices: match advices.len() {
				0 => None,
				_ => Some(advices)
			}
		}
	}

	pub fn show_advices(&self) {
		if let Some(ref advices) = self.advices {
	    	info!("Advices:");
	    	for advice in advices {
	    		info!(" 💡 {}", advice);
	    	}
		} else {
			info!("No advice.");
		}
	}
}
