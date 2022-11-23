use objc2::{extern_protocol, ProtocolType};

extern_protocol!(
    pub(crate) struct NSTextInputClient;

    unsafe impl ProtocolType for NSTextInputClient {
        // TODO: Methods
    }
);
