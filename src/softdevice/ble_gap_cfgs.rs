use crate::softdevice::{self as sd, bindgen};

pub fn role_count(cfg: bindgen::ble_gap_cfg_role_count_t) -> Result<(), ()> {
    sd::ble_cfg_set(
        bindgen::BLE_GAP_CFGS_BLE_GAP_CFG_ROLE_COUNT,
        bindgen::ble_cfg_t {
            gap_cfg: bindgen::ble_gap_cfg_t {
                role_count_cfg: cfg,
            },
        },
    )
}

pub fn device_name(cfg: bindgen::ble_gap_cfg_device_name_t) -> Result<(), ()> {
    sd::ble_cfg_set(
        bindgen::BLE_GAP_CFGS_BLE_GAP_CFG_DEVICE_NAME,
        bindgen::ble_cfg_t {
            gap_cfg: bindgen::ble_gap_cfg_t {
                device_name_cfg: cfg,
            },
        },
    )
}
