#[doc = "Register `SAR_PERI_CLK_GATE_CONF` reader"]
pub type R = crate::R<SAR_PERI_CLK_GATE_CONF_SPEC>;
#[doc = "Register `SAR_PERI_CLK_GATE_CONF` writer"]
pub type W = crate::W<SAR_PERI_CLK_GATE_CONF_SPEC>;
#[doc = "Field `SAR_RTC_I2C_CLK_EN` reader - enable rtc i2c clock"]
pub type SAR_RTC_I2C_CLK_EN_R = crate::BitReader;
#[doc = "Field `SAR_RTC_I2C_CLK_EN` writer - enable rtc i2c clock"]
pub type SAR_RTC_I2C_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_TSENS_CLK_EN` reader - enable tsens clock"]
pub type SAR_TSENS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_CLK_EN` writer - enable tsens clock"]
pub type SAR_TSENS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_SARADC_CLK_EN` reader - enbale saradc clock"]
pub type SAR_SARADC_CLK_EN_R = crate::BitReader;
#[doc = "Field `SAR_SARADC_CLK_EN` writer - enbale saradc clock"]
pub type SAR_SARADC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_IOMUX_CLK_EN` reader - enable io_mux clock"]
pub type SAR_IOMUX_CLK_EN_R = crate::BitReader;
#[doc = "Field `SAR_IOMUX_CLK_EN` writer - enable io_mux clock"]
pub type SAR_IOMUX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - enable rtc i2c clock"]
    #[inline(always)]
    pub fn sar_rtc_i2c_clk_en(&self) -> SAR_RTC_I2C_CLK_EN_R {
        SAR_RTC_I2C_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - enable tsens clock"]
    #[inline(always)]
    pub fn sar_tsens_clk_en(&self) -> SAR_TSENS_CLK_EN_R {
        SAR_TSENS_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enbale saradc clock"]
    #[inline(always)]
    pub fn sar_saradc_clk_en(&self) -> SAR_SARADC_CLK_EN_R {
        SAR_SARADC_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable io_mux clock"]
    #[inline(always)]
    pub fn sar_iomux_clk_en(&self) -> SAR_IOMUX_CLK_EN_R {
        SAR_IOMUX_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PERI_CLK_GATE_CONF")
            .field("sar_rtc_i2c_clk_en", &self.sar_rtc_i2c_clk_en())
            .field("sar_tsens_clk_en", &self.sar_tsens_clk_en())
            .field("sar_saradc_clk_en", &self.sar_saradc_clk_en())
            .field("sar_iomux_clk_en", &self.sar_iomux_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 27 - enable rtc i2c clock"]
    #[inline(always)]
    pub fn sar_rtc_i2c_clk_en(&mut self) -> SAR_RTC_I2C_CLK_EN_W<'_, SAR_PERI_CLK_GATE_CONF_SPEC> {
        SAR_RTC_I2C_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 29 - enable tsens clock"]
    #[inline(always)]
    pub fn sar_tsens_clk_en(&mut self) -> SAR_TSENS_CLK_EN_W<'_, SAR_PERI_CLK_GATE_CONF_SPEC> {
        SAR_TSENS_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - enbale saradc clock"]
    #[inline(always)]
    pub fn sar_saradc_clk_en(&mut self) -> SAR_SARADC_CLK_EN_W<'_, SAR_PERI_CLK_GATE_CONF_SPEC> {
        SAR_SARADC_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable io_mux clock"]
    #[inline(always)]
    pub fn sar_iomux_clk_en(&mut self) -> SAR_IOMUX_CLK_EN_W<'_, SAR_PERI_CLK_GATE_CONF_SPEC> {
        SAR_IOMUX_CLK_EN_W::new(self, 31)
    }
}
#[doc = "the peri clock gate of rtc peri\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_peri_clk_gate_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_peri_clk_gate_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_PERI_CLK_GATE_CONF_SPEC;
impl crate::RegisterSpec for SAR_PERI_CLK_GATE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_peri_clk_gate_conf::R`](R) reader structure"]
impl crate::Readable for SAR_PERI_CLK_GATE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_peri_clk_gate_conf::W`](W) writer structure"]
impl crate::Writable for SAR_PERI_CLK_GATE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_PERI_CLK_GATE_CONF to value 0"]
impl crate::Resettable for SAR_PERI_CLK_GATE_CONF_SPEC {}
