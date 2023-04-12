#[cfg(not(windows))]
fn main(){
    panic!("This program is only intended to run onn Windows.");
}

#[cfg(windows)]
fn main() -> windows_service::Result<()>{

    use std::ffi::OsString;
    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType},
        service_manager::{ServiceManager, ServiceManagerAccess}
    };

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    //declare the bin path of the exe of the service that we want to run daemon
    let service_bin_path = std::env::current_exe()
    .unwrap()
    .with_file_name("cron_service.exe");

    //
    let service_info = ServiceInfo{
        name: OsString::from("cron_service"),
        display_name: OsString::from("Cron Service"),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::OnDemand,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_bin_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None, //meaning use Windows System Account
        account_password: None
    };

    //Actually create/ Register service on windows msvc
    let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
    service.set_description("Windows service using windows-service-rs");

    //return just Ok
    Ok(())









}