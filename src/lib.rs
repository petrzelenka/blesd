/*
 * Copyright (c) 2023, Petr Zelenka
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the attached LICENSE file or online at
 * <https://opensource.org/license/bsd-3-clause/>.
 */
use btleplug::api::CharPropFlags;
use btleplug::platform::PeripheralId;
use tabled::settings::Style;
use tabled::Tabled;
use tabled::col;
use uuid::Uuid;

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

const BASE_UUID_MASK: u128 = 0x0000_0000_0000_1000_8000_0080_5F9B_34FB_u128;

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct PeripheralDeviceDescriptor {

	#[tabled(rename = "IDENTIFIER (UUID / MAC ADDRESS)")]
	pub device_id: PeripheralId,

	#[tabled(rename = "RSSI [dBm]")]
	pub rssi: i16,

	pub name: String,

	pub manufacturer: String,

}

impl PeripheralDeviceDescriptor {

	pub fn new(device_id: PeripheralId, rssi: Option<i16>, name: Option<String>, manufacturer: Option<String>) -> Self {
		Self {
			device_id,
			rssi: rssi.unwrap_or_default(),
			name: name.unwrap_or_default(),
			manufacturer: manufacturer.unwrap_or_default(),
		}
	}
	
}

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

pub fn format_characteristic_properties(properties: CharPropFlags) -> String {
	let mut supported_properties: Vec<&str> = Vec::new();
	if properties.contains(CharPropFlags::BROADCAST) {
		supported_properties.push("broadcast");
	}
	if properties.contains(CharPropFlags::READ) {
		supported_properties.push("read");
	}
	if properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE) {
		supported_properties.push("write without response");
	}
	if properties.contains(CharPropFlags::WRITE) {
		supported_properties.push("write");
	}
	if properties.contains(CharPropFlags::NOTIFY) {
		supported_properties.push("notify");
	}
	if properties.contains(CharPropFlags::INDICATE) {
		supported_properties.push("indicate");
	}
	if properties.contains(CharPropFlags::AUTHENTICATED_SIGNED_WRITES) {
		supported_properties.push("authenticated signed writes");
	}
	if properties.contains(CharPropFlags::EXTENDED_PROPERTIES) {
		supported_properties.push("extended properties");
	}

	supported_properties.join(", ")
}

pub fn format_subitems(items: Vec<String>, item_title: &str) -> String {
	items.iter()
		.map(|item| col![item_title, item].with(Style::rounded()).to_string())
		.collect::<Vec<String>>()
		.join("\n")
}

pub fn format_uuid(uuid: Uuid) -> String {
	let bits = uuid.as_u128();
	if bits & BASE_UUID_MASK == BASE_UUID_MASK {
		return format!("{:#04x}", (bits >> 96));
	}

	uuid.to_string()
}
