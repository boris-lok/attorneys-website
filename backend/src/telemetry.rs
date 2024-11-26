use tokio::task::JoinHandle;
use tracing::Subscriber;
use tracing_subscriber::fmt::MakeWriter;

pub fn get_subscriber<Sink>(sink: Sink) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static,
{
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(sink)
        .finish();

    subscriber
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    tracing_log::LogTracer::init().expect("failed to setup log tracer");
    tracing::subscriber::set_global_default(subscriber).expect("failed to set subscriber")
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
