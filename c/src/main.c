#include "io.h"

#include "stm32f4xx_gpio.h"
#include "stm32f4xx_rcc.h"
#include "stm32f4xx_usart.h"

int main(void)
{
	RCC_AHB1PeriphClockCmd(RCC_AHB1Periph_GPIOA, ENABLE);
	RCC_APB1PeriphClockCmd(RCC_APB1Periph_UART4, ENABLE);

	GPIO_PinAFConfig(GPIOA, GPIO_PinSource0, GPIO_AF_UART4);
	GPIO_PinAFConfig(GPIOA, GPIO_PinSource1, GPIO_AF_UART4);

	GPIO_InitTypeDef gpio_init_options = {
		.GPIO_Pin = GPIO_Pin_0 | GPIO_Pin_1,
		.GPIO_Mode = GPIO_Mode_AF,
	};
	GPIO_Init(GPIOA, &gpio_init_options);

	USART_InitTypeDef uart_init_options = {
		.USART_BaudRate = 115200,
		.USART_Mode = USART_Mode_Rx | USART_Mode_Tx,
	};
	USART_Init(UART4, &uart_init_options);
	USART_Cmd(UART4, ENABLE);

	while (USART_GetFlagStatus(UART4, USART_FLAG_TC) == RESET)
		;

	const char *message = "Hello, world!\r\n";
	for (const char *p = message; *p != '\0'; ++p)
		print_char(*p);
}
