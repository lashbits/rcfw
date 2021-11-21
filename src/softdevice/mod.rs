pub use nrf_softdevice_s140 as bindgen;

pub mod ble_conn_cfgs;
pub mod ble_gap_cfgs;
pub mod ble_gatts_cfgs;

pub const APP_CONN_CFG_TAG: u8 = 1;

pub fn app_ram_base() -> u32 {
    unsafe {
        extern "C" {
            /* https://interrupt.memfault.com/blog/zero-to-main-rust-1#a-little-help-from-the-linker
             * https://github.com/ferrous-systems/zero-to-main/blob/1b6e00ce8b10d9c7a4832083ba7bb0646cf3821d/from-scratch/src/main.rs#L25
             */
            static mut __sdata: u32; // Start of .data section
        }
        &mut __sdata as *mut u32 as u32
    }
}

unsafe extern "C" fn fault_handler(id: u32, pc: u32, info: u32) {
    match (id, info) {
        (bindgen::NRF_FAULT_ID_SD_ASSERT, _) => panic!(
            "Softdevice assertion failed: an assertion inside the softdevice's code has failed. Most common cause is disabling interrupts for too long. Make sure you're using nrf_softdevice::interrupt::free instead of cortex_m::interrupt::free, which disables non-softdevice interrupts only. PC={:x}",
            pc
        ),
        (bindgen::NRF_FAULT_ID_APP_MEMACC, 0) => panic!(
            "Softdevice memory access violation. Your program accessed RAM reserved to the softdevice. PC={:x}",
            pc
        ),
        (bindgen::NRF_FAULT_ID_APP_MEMACC, _) => panic!(
            "Softdevice memory access violation. Your program accessed registers for a peripheral reserved to the softdevice. PC={:x} PREGION={:?}",
            pc, info
        ),
        _ => panic!(
            "Softdevice unknown fault id={:?} pc={:x} info={:?}",
            id, pc, info
        ),
    }
}

pub fn enable(clock_lf_cfg: bindgen::nrf_clock_lf_cfg_t) {
    let rv: u32;
    unsafe {
        rv = bindgen::sd_softdevice_enable(
            &clock_lf_cfg as *const bindgen::nrf_clock_lf_cfg_t,
            Some(fault_handler),
        );
    }

    defmt::info!("enabling the softdevice");

    match rv {
        bindgen::NRF_SUCCESS => {
            defmt::info!("softdevice succesfully enabled");
        }
        bindgen::NRF_ERROR_INVALID_ADDR => {
            panic!("invalid or NULL pointer supplied");
        }
        bindgen::NRF_ERROR_INVALID_STATE => {
            panic!("softdevice already enabled");
        }
        bindgen::NRF_ERROR_SDM_INCORRECT_INTERRUPT_CONFIGURATION => {
            panic!("softdevice interrupt is already enabled, or an enabled interrupt has an illegal priority level");
        }
        bindgen::NRF_ERROR_SDM_LFCLK_SOURCE_UNKNOWN => {
            panic!("unknown low frequency clock source selected");
        }
        bindgen::NRF_ERROR_INVALID_PARAM => {
            panic!("invalid clock source configuration supplied in p_clock_lf_cfg")
        }
        _ => panic!("unknown error occured"),
    };
}

pub fn ble_enable(app_ram_base: &mut u32) {
    let rv: u32;
    unsafe {
        rv = bindgen::sd_ble_enable(app_ram_base as *mut u32);
    }

    defmt::info!("enabling the softdevice bluetooth stack");

    match rv {
        bindgen::NRF_SUCCESS => {
            defmt::info!("softdevice bluetooth stack succesfully enabled");
        }
        bindgen::NRF_ERROR_INVALID_STATE => {
            panic!("ble stack already initialized");
        }
        bindgen::NRF_ERROR_INVALID_ADDR => {
            panic!("invalid or not sufficiently aligned pointer supplied");
        }
        bindgen::NRF_ERROR_NO_MEM => {
            panic!("not enough memory");
        }
        bindgen::NRF_ERROR_RESOURCES => {
            panic!("total number of L2CAP channels configured is too large");
        }
        _ => panic!("unknown error occured"),
    };
}

/// Apply a configuration to the softdevice.
///
/// Configurations items are categorized into four main parts, found via the union types
///     BLE_CONN_CFGS
///     BLE_COMMON_CFGS
///     BLE_GAP_CFGS
///     BLE_GATTS_CFGS
///
/// These union types each include multiple sub configuration items. For example, the BLE_CONN_CFGS union includes
///     BLE_CONN_CFG_GAP
///     BLE_CONN_CFG_GATTC
///     BLE_CONN_CFG_GATTS
///     BLE_CONN_CFG_GATT
///     BLE_CONN_CFG_L2CAP
///
/// For each configuration, there is an associated member in the ble_cfg_t union. These members are
///     ble_conn_cfg_t          conn_cfg        (for BLE_CONN_CFGS)
///     ble_common_cfg_t        common_cfg      (for BLE_COMMON_CFGS)
///     ble_gap_cfg_t           gap_cfg         (for BLE_GAP_CFGS)
///     ble_gatts_cfg_t         gatts_cfg       (for BLE_GATTS_CFGS)
///
/// These members are themselves unions, where each member is a struct for the associated subconfiguration. For example,
/// the ble_conn_cfg_t union has the members
///     ble_gap_conn_cfg_t      gap_conn_cfg    (for BLE_CONN_CFG_GAP)
///     ble_gattc_conn_cfg_t    gattc_conn_cfg  (for BLE_CONN_CFG_GATTC)
///     ble_gatts_conn_cfg_t    gatts_conn_cfg  (for BLE_CONN_CFG_GATTS)
///     ble_gatt_conn_cfg_t     gatt_conn_cfg   (for BLE_CONN_CFG_GATT)
///     ble_l2cap_conn_cfg_t    l2cap_conn_cfg  (for BLE_CONN_CFG_L2CAP)
pub fn ble_cfg_set(cfg_id: u32, cfg: bindgen::ble_cfg_t) {
    let rv: u32;
    unsafe {
        rv = bindgen::sd_ble_cfg_set(cfg_id, &cfg as *const bindgen::ble_cfg_t, app_ram_base());
    }

    match rv {
        bindgen::NRF_SUCCESS => {}
        bindgen::NRF_ERROR_INVALID_STATE => {
            panic!("BLE stack had already been initialized");
        }
        bindgen::NRF_ERROR_INVALID_ADDR => {
            panic!("invalid or not sufficiently aligned pointer supplied");
        }
        bindgen::NRF_ERROR_INVALID_PARAM => {
            panic!("invalid cfg_id supplied");
        }
        bindgen::NRF_ERROR_NO_MEM => panic!("not enough memory"),
        _ => panic!("unknown error occured"),
    };
}
