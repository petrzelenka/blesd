/*
 * Copyright (c) 2023, Petr Zelenka
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the attached LICENSE file or online at
 * <https://opensource.org/license/bsd-3-clause/>.
 */
use uuid::Uuid;

use phf::phf_map;

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

// Data provided by Bluetooth SIG
// see https://www.bluetooth.com/specifications/assigned-numbers/

static UUIDS: phf::Map<u128, &'static str> = phf_map! {
	0x0000290000001000800000805F9B34FBu128 => "Characteristic Extended Properties",
	0x0000290100001000800000805F9B34FBu128 => "Characteristic User Description",
	0x0000290200001000800000805F9B34FBu128 => "Client Characteristic Configuration",
	0x0000290300001000800000805F9B34FBu128 => "Server Characteristic Configuration",
	0x0000290400001000800000805F9B34FBu128 => "Characteristic Presentation Format",
	0x0000290500001000800000805F9B34FBu128 => "Characteristic Aggregate Format",
	0x0000290600001000800000805F9B34FBu128 => "Valid Range",
	0x0000290700001000800000805F9B34FBu128 => "External Report Reference",
	0x0000290800001000800000805F9B34FBu128 => "Report Reference",
	0x0000290900001000800000805F9B34FBu128 => "Number of Digitals",
	0x0000290A00001000800000805F9B34FBu128 => "Value Trigger Setting",
	0x0000290B00001000800000805F9B34FBu128 => "Environmental Sensing Configuration",
	0x0000290C00001000800000805F9B34FBu128 => "Environmental Sensing Measurement",
	0x0000290D00001000800000805F9B34FBu128 => "Environmental Sensing Trigger Setting",
	0x0000290E00001000800000805F9B34FBu128 => "Time Trigger Setting",
	0x0000290F00001000800000805F9B34FBu128 => "Complete BR-EDR Transport Block Data",
	0x0000291000001000800000805F9B34FBu128 => "Observation Schedule",
	0x0000291100001000800000805F9B34FBu128 => "Valid Range and Accuracy",
};

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

pub fn find_descriptor_type(uuid: Uuid) -> String {
	UUIDS.get(&uuid.as_u128()).unwrap_or(&"unknown").to_string()
}
