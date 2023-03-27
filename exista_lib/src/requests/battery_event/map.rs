use crate::application::constants::*;

pub trait Map{
    fn map(self)->i32;
}
impl Map for u16{
    /// matches input event code with JsonValue
    fn map(self)->i32{
        match self{
            BATT_IC_OK => 7,                    // BATT_IC_OK
            BATT_IC_SUPPLY_BYDC => 2,           // BATT_IC_SUPPLY_BYDC
            BATT_IC_SUPPLY_BYBATT => 1,         // BATT_IC_SUPPLY_BYBATT
            BATT_IC_CHARGING => 9,              // BATT_IC_CHARGING
            BATT_IC_LOW => 6,                   // BATT_IC_LOW 
            BATT_IC_DISCHARGED => 8,            // BATT_IC_DISCHARGED
            BATT_IC_MISSING => 3,               // BATT_IC_MISSING
            BATT_IC_DEFECT => 5,                // BATT_IC_DEFECT
            BATT_IC_AGED => 4,                  // BATT_IC_AGED
            BATT_IC_SHAKE => 5,                 // BATT_IC_DEFECT
            BATT_IC_CHARGED => 9,               // BATT_IC_CHARGING
            BATT_IC_REBOOT => DONT_FORWARD,     // DONT_FORWARD!
            BATT_IC_CHARGING_NO_CHANGE => 9,    // BATT_IC_CHARGING
            BATT_IC_CHARG_OVER_CURRENT => 5,    // BATT_IC_DEFECT
            BATT_IC_OVER_VLOT => 5,             // BATT_IC_DEFECT
            BATT_IC_OVER_TEMPERATURE => DONT_FORWARD,   // DONT_FORWARD!
            BATT_IC_LOW_TEMPERATURE => DONT_FORWARD,    // DONT_FORWARD!
            BATT_IC_EQUAGL_CHARG_TOOLONG => 4,          // BATT_IC_AGED  
            BATT_IC_DEFECT_DISCHARG_OVER_CURR => 8,     // BATT_IC_DISCHARGED
            _=> DONT_FORWARD                            // DONT_FORWARD!
        }
    }
}