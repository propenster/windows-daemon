#[cfg(not(windows))]
fn main(){
    panic!("This program is only intended to run on Windows.");
}

#[cfg(windows)]
fn main() -> windows_service::Result<()>{

    use std::{thread, time::Duration};
    use windows_service::{
        service::{ServiceAccess, ServiceState},
        service_manager::{ServiceManager, ServiceManagerAccess}
    };

    //create handle for sys management access...
    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    //define service access 
    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;

    let service = service_manager.open_service("cron_service", service_access)?;
    
    let service_status = service.query_status()?;

    println!("Service is currently {:?}", service_status);
    if service_status.current_state != ServiceState::Stopped{
        service.stop()?;
        //wait 1 second for service to stop
        thread::sleep(Duration::from_secs(1));
    }

    println!("Service is now stopped...");
    service.delete()?;
    Ok(());




}