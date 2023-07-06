unsafe fn drop_slab_in_place(value: &mut AllocSlab) {
    use crate::parser::char_reader::CharReader;

    match value.header.tag() {
        ArenaHeaderTag::Integer => {
            ptr::drop_in_place(value.payload_offset::<Integer>());
        }
        ArenaHeaderTag::Rational => {
            ptr::drop_in_place(value.payload_offset::<Rational>());
        }
        ArenaHeaderTag::InputFileStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<CharReader<InputFileStream>>>());
        }
        ArenaHeaderTag::OutputFileStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<OutputFileStream>>());
        }
        ArenaHeaderTag::NamedTcpStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<CharReader<NamedTcpStream>>>());
        }
        ArenaHeaderTag::NamedTlsStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<CharReader<NamedTlsStream>>>());
        }
        ArenaHeaderTag::HttpReadStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<CharReader<HttpReadStream>>>());
        }
	    ArenaHeaderTag::HttpWriteStream => {
	        ptr::drop_in_place(value.payload_offset::<StreamLayout<CharReader<HttpWriteStream>>>());
	    }
        ArenaHeaderTag::ReadlineStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<ReadlineStream>>());
        }
        ArenaHeaderTag::StaticStringStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<StaticStringStream>>());
        }
        ArenaHeaderTag::ByteStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<CharReader<ByteStream>>>());
        }
        ArenaHeaderTag::OssifiedOpDir => {
            ptr::drop_in_place(value.payload_offset::<OssifiedOpDir>());
        }
        ArenaHeaderTag::LiveLoadState | ArenaHeaderTag::InactiveLoadState => {
            ptr::drop_in_place(value.payload_offset::<LiveLoadState>());
        }
        ArenaHeaderTag::Dropped => {
        }
        ArenaHeaderTag::TcpListener => {
            ptr::drop_in_place(value.payload_offset::<TcpListener>());
        }
	    ArenaHeaderTag::HttpListener => {
	        ptr::drop_in_place(value.payload_offset::<HttpListener>());
	    }
	    ArenaHeaderTag::HttpResponse => {
	        ptr::drop_in_place(value.payload_offset::<HttpResponse>());
	    }
        ArenaHeaderTag::StandardOutputStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<StandardOutputStream>>());
        }
        ArenaHeaderTag::StandardErrorStream => {
            ptr::drop_in_place(value.payload_offset::<StreamLayout<StandardErrorStream>>());
        }
        ArenaHeaderTag::NullStream | ArenaHeaderTag::IndexPtrUndefined |
        ArenaHeaderTag::IndexPtrDynamicUndefined | ArenaHeaderTag::IndexPtrDynamicIndex |
        ArenaHeaderTag::IndexPtrIndex => {
        }
    }
}
// https://github.com/mthom/scryer-prolog/blob/b5b45dde9d748644e16cefac74deae66fa50f26e/src/arena.rs#L705