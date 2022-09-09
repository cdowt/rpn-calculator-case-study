#include "io.h"

#include "stm32f4xx_usart.h"

void print_str(const char *str)
{
	for (const char *p = str; *p != '\0'; ++p)
		print_char(*p);
}

void print_char(char c)
{
    USART_SendData(UART4, (uint16_t)c);
    while (USART_GetFlagStatus(UART4, USART_FLAG_TC) == RESET)
        ;
}

char read_char(void)
{
    while (USART_GetFlagStatus(UART4, USART_FLAG_RXNE))
        ;
    return (char)USART_ReceiveData(UART4);
}
