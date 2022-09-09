CFLAGS += -mlittle-endian -mthumb -mcpu=cortex-m4
CFLAGS += -mfloat-abi=hard -mfpu=fpv4-sp-d16
CFLAGS += -fno-exceptions --specs=nosys.specs

CFLAGS += -I third-party
CFLAGS += -I third-party/CMSIS/Device/ST/STM32F4xx/Include
CFLAGS += -I third-party/CMSIS/Include
CFLAGS += -I third-party/STM32F4xx_StdPeriph_Driver/inc

CFLAGS += -D USE_STDPERIPH_DRIVER
CFLAGS += -D STM32F40_41xxx
CFLAGS += -D HSE_VALUE=8000000

LDFLAGS += -T third-party/STM32F417IG_FLASH.ld

STDPERIPH_SRC = \
	third-party/STM32F4xx_StdPeriph_Driver/src/misc.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_adc.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_can.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_crc.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_cryp.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_cryp_aes.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_cryp_des.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_cryp_tdes.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_dac.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_dbgmcu.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_dcmi.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_dma.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_exti.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_flash.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_flash_ramfunc.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_fsmc.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_gpio.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_hash.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_hash_md5.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_hash_sha1.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_i2c.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_iwdg.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_pwr.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_rcc.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_rng.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_rtc.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_sdio.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_spi.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_tim.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_usart.c \
	third-party/STM32F4xx_StdPeriph_Driver/src/stm32f4xx_wwdg.c
STDPERIPH_OBJ = $(STDPERIPH_SRC:.c=.o)

STARTUP_OBJ = \
	third-party/CMSIS/Device/ST/STM32F4xx/Source/Templates/system_stm32f4xx.o \
	third-party/CMSIS/Device/ST/STM32F4xx/Source/Templates/gcc_ride7/startup_stm32f40xx.o

DEVICE_LIBS = bin/stdperiph.a bin/startup.a

bin/stdperiph.a: $(STDPERIPH_OBJ)
	$(AR) rs $@ $(STDPERIPH_OBJ)

bin/startup.a: $(STARTUP_OBJ)
	$(AR) rs $@ $(STARTUP_OBJ)
