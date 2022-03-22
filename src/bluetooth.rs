use crate::softdevice as sd;
use core::mem;
use defmt_rtt as _;

pub fn enable() {
    sd::enable(sd::bindgen::nrf_clock_lf_cfg_t {
        source: sd::bindgen::NRF_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: 4,
        rc_temp_ctiv: 2,
        accuracy: 7,
    });
}

pub fn init() {
    sd::ble_conn_cfgs::gap(sd::bindgen::ble_gap_conn_cfg_t {
        conn_count: 6,
        event_length: 6,
    });
    sd::ble_conn_cfgs::gatt(sd::bindgen::ble_gatt_conn_cfg_t { att_mtu: 128 });

    sd::ble_gap_cfgs::role_count(sd::bindgen::ble_gap_cfg_role_count_t {
        adv_set_count: 1,
        periph_role_count: 3,
        central_role_count: 3,
        central_sec_count: 0,
        _bitfield_1: sd::bindgen::ble_gap_cfg_role_count_t::new_bitfield_1(0),
    });
    sd::ble_gap_cfgs::device_name(sd::bindgen::ble_gap_cfg_device_name_t {
        p_value: b"HelloRust" as *const u8 as _,
        current_len: 9,
        max_len: 9,
        write_perm: unsafe { mem::zeroed() },
        _bitfield_1: sd::bindgen::ble_gap_cfg_device_name_t::new_bitfield_1(
            sd::bindgen::BLE_GATTS_VLOC_STACK as u8,
        ),
    });

    sd::ble_gatts_cfgs::attr_tab_size(sd::bindgen::ble_gatts_cfg_attr_tab_size_t {
        attr_tab_size: 32768,
    });

    let mut wanted_app_ram_base = sd::app_ram_base();
    sd::ble_enable(&mut wanted_app_ram_base);
    defmt::info!("wanted app ram base is {:x}", wanted_app_ram_base);
}

pub fn connect(address: [u8; 6]) {
    sd::ble_gap_tx_power_set(
        sd::bindgen::BLE_GAP_TX_POWER_ROLES_BLE_GAP_TX_POWER_ROLE_SCAN_INIT as _,
        0,
        0,
    );

    let mut buf = [0u8; sd::bindgen::BLE_GAP_SCAN_BUFFER_MAX as usize];
    sd::ble_gap_scan_start(
        sd::bindgen::ble_gap_scan_params_t {
            _bitfield_1: sd::bindgen::ble_gap_cfg_role_count_t::new_bitfield_1(0),
            scan_phys: sd::bindgen::BLE_GAP_PHY_1MBPS as _,
            interval: 2732,
            window: 500,
            timeout: sd::bindgen::BLE_GAP_SCAN_TIMEOUT_UNLIMITED as _,
            channel_mask: [0; 5],
        },
        &mut buf,
    );
    defer!(sd::ble_gap_scan_stop());
}
