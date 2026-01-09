#![no_std]
#![no_main]

// パニック時にCPUを停止させる（デバッグ用）
use panic_halt as _;

// RP2040用HAL（ハードウェア抽象化ライブラリ）
use hal::pac;
use rp2040_hal as hal;

// OutputPinトレイトでGPIO出力を制御
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::clocks::Clock;

// ブートローダ配置（RP2040の起動に必須）
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// 外部水晶発振子の周波数（12MHz）
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// エントリポイント（main関数）
#[rp2040_hal::entry]
fn main() -> ! {
    // 周辺機能のハンドルを取得
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // ウォッチドッグタイマ初期化
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // クロック初期化（システム動作に必須）
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // 遅延用タイマ（ms単位の待ち処理に使用）
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // SIO（Single-cycle IO）初期化
    let sio = hal::Sio::new(pac.SIO);

    // GPIOピン群の初期化
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // LED（GPIO15）を出力に設定
    let mut led_pin = pins.gpio15.into_push_pull_output();

    // メインループ：LEDを500msごとにON/OFF
    loop {
        // LED点灯
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        // LED消灯
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
