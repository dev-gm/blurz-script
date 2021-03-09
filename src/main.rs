pub use blurz::{
    bluetooth_session::BluetoothSession,
    bluetooth_adapter::BluetoothAdapter,
    bluetooth_device::BluetoothDevice,
    bluetooth_discovery_session::BluetoothDiscoverySession,
};

fn search() -> BluetoothSession {
    let session = BluetoothSession::create_session(None).unwrap();
    let adapter = BluetoothAdapter::init(&session).unwrap();
    if let Err(err) = adapter.set_powered(true) {
        panic!("Cannot power on: {:?}", err);
    }
    if let Err(err) = adapter.set_pairable(true) {
        panic!("Cannot set pairable {:?}", err);
    }
    let discovery_session = BluetoothDiscoverySession::create_session(&session, adapter.get_id()).unwrap();
    session
}

fn main() {
    let ip = "E8_07_BF_F6_6D_0D";
    let address = String::from(format!("/org/bluez/hci0/dev_{}", ip));
    let session = search();
    let device = BluetoothDevice::new(&session, address);
    if let Err(err) = device.connect(10000) {
        println!("Failed to connect: {:?}", err);
    }
}
