#[macro_export]
macro_rules! impl_uow {
    ($service:ident) => {
        pub struct $service<F: UnitOfWorkFactory> {
            factory: F,
        }

        impl<F: UnitOfWorkFactory> $service<F> {
            pub fn new(factory: F) -> Self {
                Self { factory }
            }
        }
    };
}
