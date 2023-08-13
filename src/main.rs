/*
 * Copyright (c) 2023, Petr Zelenka
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the attached LICENSE file or online at
 * <https://opensource.org/license/bsd-3-clause/>.
 */
use std::error::Error;
use std::process::exit;
use std::time::Duration;

use btleplug::api::Central;
use btleplug::api::Manager as _;
use btleplug::api::Peripheral as _;
use btleplug::api::ScanFilter;
use btleplug::platform::Manager;
use clap::Parser;
use clap::Subcommand;
use colored::Colorize;
use tabled::settings::object::Columns;
use tabled::settings::Alignment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::Table;
use tokio::time;

use blesd::PeripheralDeviceDescriptor;
use blesd::format_characteristic_properties;
use blesd::format_subitems;
use blesd::format_uuid;

pub mod characteristic_uuids;
use characteristic_uuids::find_characteristic_type;

pub mod descriptor_uuids;
use descriptor_uuids::find_descriptor_type;

pub mod manufacturer_ids;
use manufacturer_ids::find_manufacturer;

pub mod service_uuids;
use service_uuids::find_service_type;

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

const DEFAULT_SCAN_TIME: u64 = 11;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {

	/// Discover services provided by a peripheral device
	Discover {
		#[arg()]
		/// Identifier (UUID on macOS, MAC address on other systems) of the peripheral device
		device_id: String,

		#[arg(long, short = 't', default_value_t = DEFAULT_SCAN_TIME)]
		/// Scan time in seconds
		scan_time: u64,
	},

	/// Scan for peripheral devices
	Scan {
		#[arg(long, short = 'r', default_value_t = false)]
		/// Sort devices by RSSI (strongest to weakest) instead of by identifier
		sort_by_rssi: bool,

		#[arg(long, short = 't', default_value_t = DEFAULT_SCAN_TIME)]
		/// Scan time in seconds
		scan_time: u64,
	},

}

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args = Args::parse();

	match &args.command {
		Command::Discover { device_id, scan_time } => {
			let adapters = Manager::new().await?.adapters().await?;
			if adapters.is_empty() {
				eprint!("{}", "error: ".red());
				eprintln!("no Bluetooth adapters found");
				exit(0);
			}
			if adapters.len() > 1 {
				eprint!("{}", "error: ".red());
				eprintln!("multiple Bluetooth adapters found, disconnect or disable redundant ones");
				exit(-1);
			}

			let adapter = adapters.first().unwrap();
			adapter.start_scan(ScanFilter::default()).await.unwrap_or_else(|_error| {
				eprint!("{}", "error: ".red());
				eprintln!("scanning peripheral devices failed (Bluetooth not enabled?)");
				exit(-1);
			});
			println!("Scanning ...");
			time::sleep(Duration::from_secs(*scan_time)).await;
			adapter.stop_scan().await?;

			let peripheral = adapter.peripherals().await?.into_iter()
				.find(|peripheral| peripheral.id().to_string() == *device_id)
				.unwrap_or_else(|| {
					eprint!("{}", "error: ".red());
					eprintln!("no peripheral device with identifier '{}' found", device_id.yellow());
					exit(0);
				});

			println!("Connecting ...");
			peripheral.connect().await.unwrap_or_else(|_error| {
				eprint!("{}", "error: ".red());
				eprintln!("connecting device failed (device already connected by someone else?)");
				exit(-1);
			});
	
			println!("Discovering services ...");
			peripheral.discover_services().await.unwrap_or_else(|_error| {
				eprint!("{}", "error: ".red());
				eprintln!("discovering services failed");
				exit(-1);
			});

			let mut services: Vec<String> = Vec::new();
			for service in peripheral.services() {
				let mut characteristics: Vec<String> = Vec::new();
				for characteristic in service.characteristics {
					let mut descriptors: Vec<String> = Vec::new();
					for descriptor in characteristic.descriptors {
						descriptors.push(format!("uuid: {}\ntype: {}",
							format_uuid(descriptor.uuid),
							find_descriptor_type(descriptor.uuid)
						));
					}

					let mut output = format!("uuid: {}\ntype: {}\nproperties: {}",
						format_uuid(characteristic.uuid),
						find_characteristic_type(characteristic.uuid),
						format_characteristic_properties(characteristic.properties));
					if !descriptors.is_empty() {
						output = format!("{}\n{}",
							output,
							format_subitems(descriptors, "DESCRIPTOR"));
					}
					characteristics.push(output);
				}

				let mut output = format!("uuid: {}\ntype: {}\nprimary: {}",
					format_uuid(service.uuid),
					find_service_type(service.uuid),
					service.primary);
				if !characteristics.is_empty() {
					output = format!("{}\n{}",
						output,
						format_subitems(characteristics, "CHARACTERISTIC"));
				}
				services.push(output);
			}

			let mut device: Vec<String> = Vec::new();
			device.push(format!("identifier: {}\n{}",
				peripheral.id(),
				format_subitems(services, "SERVICE")
			));

			println!("\n{}", format_subitems(device, "PERIPHERAL DEVICE"));

			Ok(())
		}

		Command::Scan { sort_by_rssi, scan_time } => {
			let adapters = Manager::new().await?.adapters().await?;
			if adapters.is_empty() {
				eprint!("{}", "error: ".red());
				eprintln!("no Bluetooth adapters found");
				exit(0);
			}
			if adapters.len() > 1 {
				eprint!("{}", "error: ".red());
				eprintln!("multiple Bluetooth adapters found, disconnect or disable redundant ones");
				exit(-1);
			}

			let adapter = adapters.first().unwrap();
			adapter.start_scan(ScanFilter::default()).await.unwrap_or_else(|_error| {
				eprint!("{}", "error: ".red());
				eprintln!("scanning peripheral devices failed (Bluetooth not enabled?)");
				exit(-1);
			});
			println!("Scanning ...");
			time::sleep(Duration::from_secs(*scan_time)).await;
			adapter.stop_scan().await?;

			let peripherals = adapter.peripherals().await?;
			if peripherals.is_empty() {
				eprint!("{}", "done: ".green());
				eprintln!("no peripheral devices found");
				exit(0);
			}

			let mut descriptors: Vec<PeripheralDeviceDescriptor> = Vec::new();
			for peripheral in peripherals {
				let properties = peripheral.properties().await?;
				match properties {
					Some(properties) => {
						descriptors.push(PeripheralDeviceDescriptor::new(
							peripheral.id(),
							properties.rssi,
							properties.local_name,
							find_manufacturer(properties.manufacturer_data)
						));
					},
					None => {
						descriptors.push(PeripheralDeviceDescriptor::new(
							peripheral.id(),
							None,
							None,
							None
						));
					}
				}
			}

			if *sort_by_rssi {
				descriptors.sort_by(|former, latter|
					latter.rssi.cmp(&former.rssi));
			} else {
				descriptors.sort_by(|former, latter|
					former.device_id.cmp(&latter.device_id));
			}

			println!("\n{}", Table::new(descriptors)
				.with(Style::rounded())
				.with(Modify::new(Columns::single(1)).with(Alignment::right())));

			Ok(())
		}
	}
}
