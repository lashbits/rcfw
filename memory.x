MEMORY
{
    /* NOTE K = KiBi = 1024 bytes
     * nrf52840 has
     *  1024K flash starting at address
     *  256K ram starting at address 0x20000000
     */
    FLASH : ORIGIN = 0x00027000, LENGTH = 1000K - 0x2700
    RAM : ORIGIN = 0x20020000, LENGTH = 128K
}
