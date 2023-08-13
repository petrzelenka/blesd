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
	0x0000180000001000800000805F9B34FBu128 => "Generic Access service",
	0x0000180100001000800000805F9B34FBu128 => "Generic Attribute service",
	0x0000180200001000800000805F9B34FBu128 => "Immediate Alert service",
	0x0000180300001000800000805F9B34FBu128 => "Link Loss service",
	0x0000180400001000800000805F9B34FBu128 => "Tx Power service",
	0x0000180500001000800000805F9B34FBu128 => "Current Time service",
	0x0000180600001000800000805F9B34FBu128 => "Reference Time Update service",
	0x0000180700001000800000805F9B34FBu128 => "Next DST Change service",
	0x0000180800001000800000805F9B34FBu128 => "Glucose service",
	0x0000180900001000800000805F9B34FBu128 => "Health Thermometer service",
	0x0000180A00001000800000805F9B34FBu128 => "Device Information service",
	0x0000180D00001000800000805F9B34FBu128 => "Heart Rate service",
	0x0000180E00001000800000805F9B34FBu128 => "Phone Alert Status service",
	0x0000180F00001000800000805F9B34FBu128 => "Battery service",
	0x0000181000001000800000805F9B34FBu128 => "Blood Pressure service",
	0x0000181100001000800000805F9B34FBu128 => "Alert Notification service",
	0x0000181200001000800000805F9B34FBu128 => "Human Interface Device service",
	0x0000181300001000800000805F9B34FBu128 => "Scan Parameters service",
	0x0000181400001000800000805F9B34FBu128 => "Running Speed and Cadence service",
	0x0000181500001000800000805F9B34FBu128 => "Automation IO service",
	0x0000181600001000800000805F9B34FBu128 => "Cycling Speed and Cadence service",
	0x0000181800001000800000805F9B34FBu128 => "Cycling Power service",
	0x0000181900001000800000805F9B34FBu128 => "Location and Navigation service",
	0x0000181A00001000800000805F9B34FBu128 => "Environmental Sensing service",
	0x0000181B00001000800000805F9B34FBu128 => "Body Composition service",
	0x0000181C00001000800000805F9B34FBu128 => "User Data service",
	0x0000181D00001000800000805F9B34FBu128 => "Weight Scale service",
	0x0000181E00001000800000805F9B34FBu128 => "Bond Management service",
	0x0000181F00001000800000805F9B34FBu128 => "Continuous Glucose Monitoring service",
	0x0000182000001000800000805F9B34FBu128 => "Internet Protocol Support service",
	0x0000182100001000800000805F9B34FBu128 => "Indoor Positioning service",
	0x0000182200001000800000805F9B34FBu128 => "Pulse Oximeter service",
	0x0000182300001000800000805F9B34FBu128 => "HTTP Proxy service",
	0x0000182400001000800000805F9B34FBu128 => "Transport Discovery service",
	0x0000182500001000800000805F9B34FBu128 => "Object Transfer service",
	0x0000182600001000800000805F9B34FBu128 => "Fitness Machine service",
	0x0000182700001000800000805F9B34FBu128 => "Mesh Provisioning service",
	0x0000182800001000800000805F9B34FBu128 => "Mesh Proxy service",
	0x0000182900001000800000805F9B34FBu128 => "Reconnection Configuration service",
	0x0000183A00001000800000805F9B34FBu128 => "Insulin Delivery service",
	0x0000183B00001000800000805F9B34FBu128 => "Binary Sensor service",
	0x0000183C00001000800000805F9B34FBu128 => "Emergency Configuration service",
	0x0000183D00001000800000805F9B34FBu128 => "Authorization Control service",
	0x0000183E00001000800000805F9B34FBu128 => "Physical Activity Monitor service",
	0x0000183F00001000800000805F9B34FBu128 => "Elapsed Time service",
	0x0000184000001000800000805F9B34FBu128 => "Generic Health Sensor service",
	0x0000184300001000800000805F9B34FBu128 => "Audio Input Control service",
	0x0000184400001000800000805F9B34FBu128 => "Volume Control service",
	0x0000184500001000800000805F9B34FBu128 => "Volume Offset Control service",
	0x0000184600001000800000805F9B34FBu128 => "Coordinated Set Identification service",
	0x0000184700001000800000805F9B34FBu128 => "Device Time service",
	0x0000184800001000800000805F9B34FBu128 => "Media Control service",
	0x0000184900001000800000805F9B34FBu128 => "Generic Media Control service",
	0x0000184A00001000800000805F9B34FBu128 => "Constant Tone Extension service",
	0x0000184B00001000800000805F9B34FBu128 => "Telephone Bearer service",
	0x0000184C00001000800000805F9B34FBu128 => "Generic Telephone Bearer service",
	0x0000184D00001000800000805F9B34FBu128 => "Microphone Control service",
	0x0000184E00001000800000805F9B34FBu128 => "Audio Stream Control service",
	0x0000184F00001000800000805F9B34FBu128 => "Broadcast Audio Scan service",
	0x0000185000001000800000805F9B34FBu128 => "Published Audio Capabilities service",
	0x0000185100001000800000805F9B34FBu128 => "Basic Audio Announcement service",
	0x0000185200001000800000805F9B34FBu128 => "Broadcast Audio Announcement service",
	0x0000185300001000800000805F9B34FBu128 => "Common Audio service",
	0x0000185400001000800000805F9B34FBu128 => "Hearing Access service",
	0x0000185500001000800000805F9B34FBu128 => "Telephony and Media Audio service",
	0x0000185600001000800000805F9B34FBu128 => "Public Broadcast Announcement service",
	0x0000185700001000800000805F9B34FBu128 => "Electronic Shelf Label service",
	0x0000185800001000800000805F9B34FBu128 => "Gaming Audio service (draft)",
	0x0000185900001000800000805F9B34FBu128 => "Mesh Proxy Solicitation service (draft)",
};

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------

pub fn find_service_type(uuid: Uuid) -> String {
	UUIDS.get(&uuid.as_u128()).unwrap_or(&"unknown").to_string()
}
