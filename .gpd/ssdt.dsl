/*
 * Intel ACPI Component Architecture
 * AML/ASL+ Disassembler version 20200925 (64-bit version)
 * Copyright (c) 2000 - 2020 Intel Corporation
 * 
 * Disassembling to symbolic ASL+ operators
 *
 * Disassembly of gpd.ssdt.aml.bin, Fri Aug 16 04:33:31 2024
 *
 * Original Table Header:
 *     Signature        "SSDT"
 *     Length           0x000000B1 (177)
 *     Revision         0x01
 *     Checksum         0x5D
 *     OEM ID           "Intel_"
 *     OEM Table ID     "ADebTabl"
 *     OEM Revision     0x00001000 (4096)
 *     Compiler ID      "INTL"
 *     Compiler Version 0x20160930 (538315056)
 */
DefinitionBlock ("", "SSDT", 1, "Intel_", "ADebTabl", 0x00001000)
{
    Scope (\)
    {
        Name (DPTR, 0x79BCD000)
        Name (EPTR, 0x79BDD000)
        Name (CPTR, 0x79BCD010)
        Mutex (MMUT, 0x00)
        Method (MDBG, 1, Serialized)
        {
            Local0 = Acquire (MMUT, 0x03E8)
            If ((Local0 == Zero))
            {
                OperationRegion (ABLK, SystemMemory, CPTR, 0x10)
                Field (ABLK, ByteAcc, NoLock, Preserve)
                {
                    AAAA,   128
                }

                AAAA = Arg0
                CPTR += 0x10
                If ((CPTR >= EPTR))
                {
                    CPTR = (DPTR + 0x10)
                }

                Release (MMUT)
            }

            Return (Local0)
        }
    }
}

