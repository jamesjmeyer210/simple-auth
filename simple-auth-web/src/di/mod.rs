mod service_factory;

pub use service_factory::DataFactory as DataFactory;
pub use service_factory::TransientFactory as TransientFactory;

pub type ServiceFactory<'r> = service_factory::ServiceFactory<'r>;