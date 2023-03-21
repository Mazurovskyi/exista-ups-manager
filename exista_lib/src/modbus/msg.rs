use json::JsonValue;
pub struct ModbusMsg{
    msg: Vec<u8>,
    len: usize
}
impl ModbusMsg{
    /// creates a new modbus message.
    pub fn from<T: IntoMsg + ?Sized>(data: &T, len: usize)->Self{
        data.into_modbus_msg(len)
    }

    pub fn data(&self)->&[u8]{
        self.msg.as_slice()
    }
    
    pub fn len(&self)->&usize{
        &self.len
    }

    pub fn registers_value(msg: &[u8])->JsonValue{
        let temp = ((msg[3] as u32) << 8) + (msg[4] as u32);
        (temp as i32).into()
    }

    pub fn registers_value_percent(msg: &[u8])->JsonValue{
        let mut temp = ((msg[3] as u32) << 8) + (msg[4] as u32);
        temp = (temp * 100) / 0xFFFF;
        (temp as i32).into()
    }

    pub fn is_event(&self)->bool{
        // event message begins with 0x0, 0x64.
        if self.data()[..2] == [0x00, 0x64]{
            true
        }
        else{
            false
        }
    }

}



pub trait IntoMsg {

    fn into_modbus_msg(&self, len: usize)->ModbusMsg;

    /// standart 16-bit crc
    fn crc(data: &[u8])->u16{
        let table:[u16;2] = [ 0x0000, 0xA001];
        let mut crc = 0xFFFF as u16;
        let mut xor = 0;

        for el in data{
            crc^=*el as u16;
            for _ in 0..8{
                xor = crc & 0x01;
                crc>>=1;
                crc^=table[xor as usize]
            }
        }
        crc
    }
}

impl IntoMsg for [u16]{

    fn into_modbus_msg(&self, len: usize)->ModbusMsg{

        let mut msg = Vec::new();

        self[..2].iter().for_each(|el|msg.push(*el as u8));
        self[2..].iter().for_each(|el|msg.extend(el.to_be_bytes()));

        let mut crc = Self::crc(msg.as_slice()).to_be_bytes();
        crc.reverse();

        msg.extend(crc);

        ModbusMsg{msg, len}
    }
}

impl IntoMsg for [u8] {
    fn into_modbus_msg(&self, len: usize)->ModbusMsg{
        let msg = Vec::from(self);
        ModbusMsg{msg, len}
    }
}


