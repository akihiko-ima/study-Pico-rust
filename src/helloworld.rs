#![no_std]
#![no_main]

use cortex_m_rt::entry; // エントリポイント属性
use panic_halt as _; // パニック時に停止
use rp_pico::hal::{clocks::init_clocks_and_plls, pac, usb::UsbBus, watchdog::Watchdog}; // RP2040用HAL
use usb_device::{class_prelude::*, prelude::*}; // USBデバイス汎用
use usbd_serial::{SerialPort, USB_CLASS_CDC}; // USB CDCクラス

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // ウォッチドッグタイマ初期化
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // クロック初期化（外部水晶発振子使用）
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ, // 水晶発振子周波数
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // USBバスの初期化
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true, // force_vbus_detect: 開発ボードでVBUS検出を強制
        &mut pac.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("pico company") // メーカー名
        .product("port") // 製品名
        .serial_number("TEST") // シリアル番号
        .device_class(USB_CLASS_CDC) // CDCクラス指定
        .build();

    let hello = b"hello world\r\n";

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        let _ = serial.write(hello);
    }
}
