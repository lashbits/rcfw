use crate::softdevice::{self as sd, bindgen};

pub fn gap(cfg: bindgen::ble_gap_conn_cfg_t) -> Result<(), ()> {
    sd::ble_cfg_set(
        bindgen::BLE_CONN_CFGS_BLE_CONN_CFG_GAP,
        bindgen::ble_cfg_t {
            conn_cfg: bindgen::ble_conn_cfg_t {
                conn_cfg_tag: sd::APP_CONN_CFG_TAG,
                params: bindgen::ble_conn_cfg_t__bindgen_ty_1 { gap_conn_cfg: cfg },
            },
        },
    )
}

pub fn gatt(cfg: bindgen::ble_gatt_conn_cfg_t) -> Result<(), ()> {
    sd::ble_cfg_set(
        bindgen::BLE_CONN_CFGS_BLE_CONN_CFG_GATT,
        bindgen::ble_cfg_t {
            conn_cfg: bindgen::ble_conn_cfg_t {
                conn_cfg_tag: sd::APP_CONN_CFG_TAG,
                params: bindgen::ble_conn_cfg_t__bindgen_ty_1 { gatt_conn_cfg: cfg },
            },
        },
    )
}
