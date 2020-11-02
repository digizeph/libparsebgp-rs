use parsebgp_rs::*;
use std::{io::Read, fs::File};

/// ```rust
///    pub fn parsebgp_decode(
///        opts: parsebgp_opts_t,
///        type_: parsebgp_msg_type_t,
///        msg: *mut parsebgp_msg_t,
///        buffer: *const u8,
///        len: *mut size_t,
///    ) -> parsebgp_error_t;
/// ```
pub fn parsebgp(file_name: &str, opts: parsebgp_opts_t, t: parsebgp_msg_type_t){
    // read file
    let mut file = File::open(file_name).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut msg = unsafe { parsebgp_create_msg() };
    let mut remain: u64 = buf.len() as u64;
    let mut bytes_read = 0;
    let mut cnt = 0;
    unsafe{
        while remain > 0 {
            let mut ptr = &buf[bytes_read];
            let mut dec_len = remain;

            let err = parsebgp_decode(opts, t, msg, ptr, &mut dec_len);
            match err {
                parsebgp_error_PARSEBGP_OK => {
                    // normal case, do nothing
                }
                parsebgp_error_PARSEBGP_PARTIAL_MSG => {
                    panic!("found partial messsage");
                }
                parsebgp_error_PARSEBGP_TRUNCATED_MSG => {
                    panic!("found truncated message");
                }
                _ => {
                    panic!("failed to parse message")
                }
            }

            assert!(dec_len>0);
            bytes_read += dec_len as usize;
            remain -= dec_len;
            cnt+=1;

            parsebgp_dump_msg(msg);
            parsebgp_clear_msg(msg);
        }
    }

    println!("INFO: read {} messages from {}", cnt, file_name);
}

//
// pub struct parsebgp_opts {
//     pub ignore_not_implemented: ::std::os::raw::c_int,
//     pub silence_not_implemented: ::std::os::raw::c_int,
//     pub ignore_invalid: ::std::os::raw::c_int,
//     pub silence_invalid: ::std::os::raw::c_int,
//     pub bgp: parsebgp_bgp_opts_t,
//     pub bmp: parsebgp_bmp_opts_t,
// }
//
// pub struct parsebgp_bgp_opts {
//     pub marker_omitted: ::std::os::raw::c_int,
//     pub marker_copy: ::std::os::raw::c_int,
//     pub asn_4_byte: ::std::os::raw::c_int,
//     pub mp_reach_no_afi_safi_reserved: ::std::os::raw::c_int,
//     pub afi: u16,
//     pub safi: u8,
//     pub path_attr_filter_enabled: ::std::os::raw::c_int,
//     pub path_attr_filter: [u8; 255usize],
//     pub path_attr_raw_enabled: ::std::os::raw::c_int,
//     pub path_attr_raw: [u8; 255usize],
// }
//
// pub struct parsebgp_bmp_opts {
//     pub peer_ip_afi: parsebgp_bgp_afi_t,
//     pub parse_headers_only: ::std::os::raw::c_int,
// }
//
// void parsebgp_opts_init(parsebgp_opts_t *opts)
// {
//   // TODO: allow the default for some of these to be configured at compile time
//   memset(opts, 0, sizeof(*opts));
//
//   parsebgp_bgp_opts_init(&opts->bgp);
// }
//
// void parsebgp_bgp_opts_init(parsebgp_bgp_opts_t *opts)
// {
//   memset(opts, 0, sizeof(*opts));
// }
fn init_opts() -> parsebgp_rs::parsebgp_opts {
    let bgp_opts = parsebgp_bgp_opts {
        marker_omitted: 0,
        marker_copy: 0,
        asn_4_byte: 0,
        mp_reach_no_afi_safi_reserved: 0,
        afi: 0,
        safi: 0,
        path_attr_filter_enabled: 0,
        path_attr_filter: [0 as u8; 255],
        path_attr_raw_enabled: 0,
        path_attr_raw: [0 as u8; 255],
    };

    let bmp_opts = parsebgp_bmp_opts {
        peer_ip_afi: parsebgp_bgp_afi_t_PARSEBGP_BGP_AFI_IPV4,
        parse_headers_only: 0,
    };

    return parsebgp_opts {
        ignore_not_implemented: 0,
        silence_not_implemented: 0,
        ignore_invalid: 0,
        silence_invalid: 0,
        bgp: bgp_opts,
        bmp: bmp_opts,
    }
}

pub fn main() {
    // TODO: initialize opts
    // TODO: parse bgp msg type
    // pub const parsebgp_msg_type_PARSEBGP_MSG_TYPE_INVALID: parsebgp_msg_type = 0;
    // pub const parsebgp_msg_type_PARSEBGP_MSG_TYPE_BGP: parsebgp_msg_type = 1;
    // pub const parsebgp_msg_type_PARSEBGP_MSG_TYPE_BMP: parsebgp_msg_type = 2;
    // pub const parsebgp_msg_type_PARSEBGP_MSG_TYPE_MRT: parsebgp_msg_type = 3;
    let opts = init_opts();
    parsebgp("/home/mingwei/Downloads/quagga_bgp", opts, 3);
    println!("this works");
}
