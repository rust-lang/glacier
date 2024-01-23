struct WSAPROTOCOLCHAIN {
    ChainEntries: [DWORD; SetFileAttributesW as usize],
}
extern "C" {
    fn SetFileAttributesW() -> BOOL;
}
