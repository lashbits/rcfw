use crate::softdevice::{self as sd, bindgen};

pub fn attr_tab_size(cfg: bindgen::ble_gatts_cfg_attr_tab_size_t) -> Result<(), ()> {
    sd::ble_cfg_set(
        bindgen::BLE_GATTS_CFGS_BLE_GATTS_CFG_ATTR_TAB_SIZE,
        bindgen::ble_cfg_t {
            gatts_cfg: bindgen::ble_gatts_cfg_t { attr_tab_size: cfg },
        },
    )
}
