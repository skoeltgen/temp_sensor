use bmp280::*;
use sqlite::*;
use chrono::{DateTime, Utc,format::SecondsFormat};
use std::path::Path;

fn main() {


	let mut sensor = Bmp280Builder::new().address(0x76).build().expect("Could not initiate Sensor");
	
    let path = Path::new("shared_data.db");
	let connection = sqlite::open(path).expect("Failed to open Database");
	
	let mut query = "SELECT name FROM sqlite_master WHERE type='table';";


    let mut init = False;

    for state in connection
		.prepare(query)
		.unwrap()//Statement
	{
        if state.unwrap()//State 
            .contains("rust_measurement")
        {
            init = True;
        }
    }

	if init { 
		query = "CREATE TABLE rust_measurement (timestamp varchar(25) PRIMARY KEY, temperature varchar(5), pressure varchar(10));"; 
		connection.execute(query).unwrap();
	}

    while True {
        let now: DateTime<Utc> = Utc::now();
	    let timestamp = now.to_rfc3339_opts(SecondsFormat::Secs, false);

        let temperature = sensor.temperature_celsius().unwrap();
        let pressure = sensor.pressure_kpa().unwrap();

        let temp_string = format!( 
			"INSERT INTO rust_measurement (timestamp, temperature, pressure) VALUES ('{}','{}','{}')",
			timestamp,
			temperature,
			pressure
		);
		query = &temp_string;
		connection.execute(query).unwrap();
    }
}