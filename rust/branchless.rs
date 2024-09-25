#[inline]
fn year_len(year: u32) -> u16 {
    let mut leap0 = year % 4;
    let mut leap = leap0 & 1;
    leap0 >>= 1;
    leap |= leap0 & 1;
    leap ^= 1;
    
    leap0 = year % 100;
    let mut leap1 = leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap &= leap1;
    
    leap0 = year % 400;
    leap1 |= leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap0 >>= 1;
    leap1 |= leap0 & 1;
    leap1 ^= 1;
    leap |= leap1;
    
    (365u32 + leap) as u16
}

#[inline]
fn year_len_br(year: u32) -> u16 {
    if (year % 4) == 0 && (year % 100) != 0 || (year % 400) == 0 {
        366u16
    } else {
        365u16
    }
}
