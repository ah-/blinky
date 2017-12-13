MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */
