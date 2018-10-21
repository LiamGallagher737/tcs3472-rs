extern crate tcs3472;
extern crate embedded_hal_mock as hal;

mod common;
use common::{ setup, check_sent_data, Register, BitFlags };

#[test]
fn can_read_rgbc_status_not_valid() {
    let mut dev = setup(&[0]);
    let is_valid = dev.is_rgbc_status_valid().unwrap();
    assert!(!is_valid);
    check_sent_data(dev, &[BitFlags::CMD | Register::STATUS]);
}

#[test]
fn can_read_rgbc_status_valid() {
    let mut dev = setup(&[BitFlags::RGBC_VALID]);
    let is_valid = dev.is_rgbc_status_valid().unwrap();
    assert!(is_valid);
    check_sent_data(dev, &[BitFlags::CMD | Register::STATUS]);
}

macro_rules! read_channel_test {
    ($name:ident, $method:ident, $register:ident) => {
        #[test]
        fn $name() {
            let mut dev = setup(&[0xCD, 0xAB]);
            let data = dev.$method().unwrap();
            assert_eq!(0xABCD, data);
            check_sent_data(dev, &[BitFlags::CMD | BitFlags::CMD_AUTO_INC | Register::$register]);
        }
    };
}

read_channel_test!(can_read_clear_channel, read_clear_channel, CDATA);
read_channel_test!(can_read_red_channel,   read_red_channel,   RDATA);
read_channel_test!(can_read_green_channel, read_green_channel, GDATA);
read_channel_test!(can_read_blue_channel,  read_blue_channel,  BDATA);

#[test]
fn can_read_device_id() {
    let mut dev = setup(&[0x44]);
    let id = dev.read_device_id().unwrap();
    assert_eq!(0x44, id);
    check_sent_data(dev, &[BitFlags::CMD | Register::ID]);
}