#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u32, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u32, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: USART mode with external clock"]
    USART_EXT_CLK,
    #[doc = "1: USART mode with internal clock"]
    USART_INT_CLK,
    #[doc = "2: SPI mode with external clock"]
    SPI_SLAVE,
    #[doc = "3: SPI mode with internal clock"]
    SPI_MASTER,
    #[doc = "4: I2C mode with external clock"]
    I2C_SLAVE,
    #[doc = "5: I2C mode with internal clock"]
    I2C_MASTER,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::USART_EXT_CLK => 0,
            MODE_A::USART_INT_CLK => 1,
            MODE_A::SPI_SLAVE => 2,
            MODE_A::SPI_MASTER => 3,
            MODE_A::I2C_SLAVE => 4,
            MODE_A::I2C_MASTER => 5,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::USART_EXT_CLK),
            1 => Val(MODE_A::USART_INT_CLK),
            2 => Val(MODE_A::SPI_SLAVE),
            3 => Val(MODE_A::SPI_MASTER),
            4 => Val(MODE_A::I2C_SLAVE),
            5 => Val(MODE_A::I2C_MASTER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODE_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODE_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODE_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODE_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODE_A::I2C_MASTER
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USART mode with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_EXT_CLK)
    }
    #[doc = "USART mode with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODE_A::USART_INT_CLK)
    }
    #[doc = "SPI mode with external clock"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODE_A::SPI_SLAVE)
    }
    #[doc = "SPI mode with internal clock"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MASTER)
    }
    #[doc = "I2C mode with external clock"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODE_A::I2C_SLAVE)
    }
    #[doc = "I2C mode with internal clock"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODE_A::I2C_MASTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PINOUT`"]
pub type PINOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINOUT`"]
pub struct PINOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PINOUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDAHOLD`"]
pub type SDAHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDAHOLD`"]
pub struct SDAHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `MEXTTOEN`"]
pub type MEXTTOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEXTTOEN`"]
pub struct MEXTTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEXTTOEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SEXTTOEN`"]
pub type SEXTTOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEXTTOEN`"]
pub struct SEXTTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEXTTOEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SCLSM`"]
pub type SCLSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLSM`"]
pub struct SCLSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLSM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `INACTOUT`"]
pub type INACTOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INACTOUT`"]
pub struct INACTOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INACTOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `LOWTOUTEN`"]
pub type LOWTOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOWTOUTEN`"]
pub struct LOWTOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWTOUTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&self) -> PINOUT_R {
        PINOUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SDAHOLD_R {
        SDAHOLD_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn mexttoen(&self) -> MEXTTOEN_R {
        MEXTTOEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&self) -> SEXTTOEN_R {
        SEXTTOEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&self) -> SCLSM_R {
        SCLSM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline(always)]
    pub fn inactout(&self) -> INACTOUT_R {
        INACTOUT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&self) -> LOWTOUTEN_R {
        LOWTOUTEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&mut self) -> PINOUT_W {
        PINOUT_W { w: self }
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&mut self) -> SDAHOLD_W {
        SDAHOLD_W { w: self }
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn mexttoen(&mut self) -> MEXTTOEN_W {
        MEXTTOEN_W { w: self }
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&mut self) -> SEXTTOEN_W {
        SEXTTOEN_W { w: self }
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&mut self) -> SCLSM_W {
        SCLSM_W { w: self }
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline(always)]
    pub fn inactout(&mut self) -> INACTOUT_W {
        INACTOUT_W { w: self }
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&mut self) -> LOWTOUTEN_W {
        LOWTOUTEN_W { w: self }
    }
}
