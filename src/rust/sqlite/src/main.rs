use bmp280::*;
use sqlite::*;
use chrono::{DateTime, Utc,format::SecondsFormat};
use std::path::Path;

fn main() {
	let mut sensor = Bmp280Builder::new().address(0x76).build().expect("Could not initiate Sensor");
	
	let now: DateTime<Utc> = Utc::now();
	//let date = now.date_naive();
	let timestamp = now.to_rfc3339_opts(SecondsFormat::Secs, false);

	//let temp_path = format!("{}.db", date.to_string());
	let path = Path::new("shared_data.db");
	let connection = sqlite::open(path).expect("Failed to open Database");
	
	let mut query = "SELECT name FROM sqlite_master WHERE type='table';";

    let temperature = sensor.temperature_celsius().unwrap();
    let pressure = sensor.pressure_kpa().unwrap();

	println!("{}: Temperatur:{} °C Luftdruck:{} kPa", timestamp,temperature,pressure);

	if let State::Done = connection
		.prepare(query)
		.unwrap()
		.next()
		.unwrap()
	{ 
		query = "CREATE TABLE measurement (timestamp varchar(25) PRIMARY KEY, temperature varchar(5), pressure varchar(10));"; 
		connection.execute(query).unwrap();
	}else{
		let temp_string = format!( 
			"INSERT INTO measurement (timestamp, temperature, pressure) VALUES ('{}','{}','{}')",
			timestamp,
			temperature,
			pressure
		);
		query = &temp_string;
		connection.execute(query).unwrap();
	}
}