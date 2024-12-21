MEMORY
{
  /* FLASH and RAM are mandatory memory regions */

  /* STM32H503 */
  FLASH   : ORIGIN = 0x08000000, LENGTH = 128K
  /* SRAM1 and SRAM2 are contiguous and can be combined into a single memory area */
  /* SRAM1   : ORIGIN = 0x20000000, LENGTH = 16K */
  /* SRAM2   : ORIGIN = 0x20004000, LENGTH = 16K */
  RAM     : ORIGIN = 0x20000000, LENGTH = 32K
  BKPSRAM : ORIGIN = 0x40036400, LENGTH = 2K
}