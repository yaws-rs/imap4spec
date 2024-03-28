use rustls::unbuffered::ConnectionState;

pub(crate) struct TlsContext;

impl TlsContext {
    pub(crate) fn new() -> Self {
        TlsContext {}
    }

    /*
        pub(crate) fn nothing_to_read<T>(state: ConnectionState<T>) -> bool {
            match state {

            }
    }
        */
}
