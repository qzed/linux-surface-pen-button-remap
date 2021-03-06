use std::fs::File;
use std::io::Result;
use std::collections::HashSet;

use evdev_rs::{Device, UInputDevice, GrabMode, ReadFlag, ReadStatus, TimeVal, InputEvent};
use evdev_rs::enums::{EventCode, EV_KEY, EV_SYN};

use nix::errno::Errno;


#[derive(Debug)]
enum EventType {
    Single,
    Double,
    Hold,
}

#[derive(Debug)]
enum EventState {
    Pressed,
    Released,
}

#[derive(Debug)]
struct Event {
    time:  TimeVal,
    ty:    EventType,
    state: EventState,
}


#[derive(Debug)]
struct Config {
    device:  DeviceMatch,
    actions: Actions,
}

#[derive(Debug)]
struct DeviceMatch {
    vendor_id:  u16,
    product_id: u16,
}

#[derive(Debug)]
struct Actions {
    single: Vec<EV_KEY>,
    double: Vec<EV_KEY>,
    hold:   Vec<EV_KEY>,
}


fn config() -> Config {
    Config {
        device: DeviceMatch {
            vendor_id:  0x045e,
            product_id: 0x0921,
        },
        actions: Actions {
            single: vec![EV_KEY::KEY_RIGHT],
            double: vec![EV_KEY::KEY_LEFT],
            hold:   vec![EV_KEY::KEY_LEFTCTRL, EV_KEY::KEY_Q],
        }
    }
}


fn match_device(config: &Config, device: &Device) -> bool {
    device.bustype() == 5 /* BUS_BLUETOOTH */ &&
    device.vendor_id() == config.device.vendor_id as _ &&
    device.product_id() == config.device.product_id as _ &&
    device.has(&EventCode::EV_KEY(EV_KEY::KEY_LEFTMETA)) &&
    device.has(&EventCode::EV_KEY(EV_KEY::KEY_F18)) &&
    device.has(&EventCode::EV_KEY(EV_KEY::KEY_F19)) &&
    device.has(&EventCode::EV_KEY(EV_KEY::KEY_F20))
}

fn find_device(config: &Config) -> Result<Option<Device>> {
    for entry in glob::glob("/dev/input/event*").unwrap() {
        let entry = entry.map_err(|e| e.into_error())?;

        let device = File::open(entry)?;
        let device = Device::new_from_fd(device)?;

        if match_device(config, &device) {
            return Ok(Some(device));
        }
    }

    Ok(None)
}

fn setup_uinput_device(config: &Config) -> Result<UInputDevice> {
    let device = Device::new().unwrap();
    device.set_name("Surface Pen Keyboard (mapped)");
    device.set_vendor_id(config.device.vendor_id as _);
    device.set_product_id(config.device.product_id as _);

    let mut keys = HashSet::new();
    keys.extend(config.actions.single.iter().cloned());
    keys.extend(config.actions.double.iter().cloned());
    keys.extend(config.actions.hold.iter().cloned());

    for key in keys {
        device.enable_event_code(&EventCode::EV_KEY(key), None)?;
    }

    Ok(UInputDevice::create_from_device(&device)?)
}

fn output_event(config: &Config, event: Event, output: &UInputDevice) -> Result<()> {
    println!("{:?}", event);    // TODO

    let value = match event.state {
        EventState::Pressed  => 1,
        EventState::Released => 0,
    };

    let keys = match event.ty {
        EventType::Single => &config.actions.single,
        EventType::Double => &config.actions.double,
        EventType::Hold   => &config.actions.hold,
    };

    for key in keys {
        let evt = InputEvent::new(&event.time, &EventCode::EV_KEY(key.clone()), value);
        output.write_event(&evt)?;
    }

    if !keys.is_empty() {
        let syn = InputEvent::new(&event.time, &EventCode::EV_SYN(EV_SYN::SYN_REPORT), 0);
        output.write_event(&syn)?;
    }

    Ok(())
}

fn handle_event_batch(config: &Config, events: &[InputEvent], output: &UInputDevice) -> Result<()> {
    if events.is_empty() {
        return Ok(());
    }

    let time = events[0].time.clone();
    let meta = events.iter().find(|e| e.event_code == EventCode::EV_KEY(EV_KEY::KEY_LEFTMETA));

    if meta.is_none() {
        return Ok(())
    }
    let meta = meta.unwrap();

    for e in events {
        let ty = match e.event_code {
            EventCode::EV_KEY(EV_KEY::KEY_F20) => EventType::Single,
            EventCode::EV_KEY(EV_KEY::KEY_F19) => EventType::Double,
            EventCode::EV_KEY(EV_KEY::KEY_F18) => EventType::Hold,
            _ => continue,
        };

        let state = if meta.value == 1 && e.value == 1 {
            EventState::Pressed
        } else if meta.value == 0 && e.value == 0 {
            EventState::Released
        } else {
            continue
        };

        let event = Event { time: time.clone(), ty, state };
        output_event(config, event, output)?;
    }

    Ok(())
}

fn handle_events(config: &Config, mut input: Device, output: UInputDevice) -> Result<()> {
    input.grab(GrabMode::Grab)?;

    let mut events = Vec::with_capacity(4);
    let mut flags = ReadFlag::NORMAL | ReadFlag::BLOCKING;
    loop { match input.next_event(flags) {
        Ok((status, evt)) => {
            flags = match status {
                ReadStatus::Success => ReadFlag::NORMAL | ReadFlag::BLOCKING,
                ReadStatus::Sync    => ReadFlag::SYNC,
            };

            match evt.event_code {
                EventCode::EV_SYN(EV_SYN::SYN_REPORT) => {
                    handle_event_batch(config, &events, &output)?;
                    events.clear();
                },
                EventCode::EV_KEY(_) => {
                    events.push(evt);
                },
                _ => {},
            };
        },
        Err(Errno::EAGAIN) => {
            flags = ReadFlag::NORMAL | ReadFlag::BLOCKING;
        },
        Err(err) => return Err(err.into()),
    }}
}

fn main() -> Result<()> {
    let config = config();

    if let Some(device) = find_device(&config)? {
        println!("Found device: '{}'", device.name().unwrap_or("<unknown>"));
        handle_events(&config, device, setup_uinput_device(&config)?)
    } else {
        Err(std::io::ErrorKind::NotFound)?
    }
}
