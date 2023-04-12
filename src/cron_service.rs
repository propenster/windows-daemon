#[cfg(not(windows))]
fn main() {
    panic!("This program is only supposed to be run on a windows machine");
}

#[cfg(windows)]

fn main() -> windows_service::Result<()> {
    ping_service::run()
}

mod ping_service {
    use std::{
        ffi::OsString,
        net::{IpAddr, SocketAddr, UdpSocket},
        sync::mpsc,
        time::Duration,
    };
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher, Result,
    };

    const SERVICE_NAME: &str = "cron_service";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;
    const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
    const RECV_ADDR: [u8; 4] = [8, 8, 8, 8]; //this is Google's public DNS...
    const RECEIVER_PORT: u16 = 1234;
    const PING_MESSAGE: &str = "ping\n";

    pub fn run() -> Result<()> {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }
    define_windows_service!(ffi_service_main, my_service_main);
    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
            // We should be handling the error here. Logging it to a file or something like that
        }
    }

    pub fn run_service() -> Result<()> {
        // Create a channel to be able to poll a stop event from the service worker loop
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Define system service event handler that will be receiving service events.
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };

        println!("Starting service...");

        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // For Demo purposes, we're just sending a message using UDP once a second
        let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
        let receiver_ip = IpAddr::from(RECV_ADDR);
        let sender_addr = SocketAddr::new(loopback_ip, 0);
        let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
        let msg = PING_MESSAGE.as_bytes();
        let socket = UdpSocket::bind(sender_addr).unwrap();

        loop {
            let _ = socket.send_to(msg, receiver_addr);

            match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
                Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            };
        }

        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        Ok(())
    }
}
