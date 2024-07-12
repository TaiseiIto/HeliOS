/*
 * Intel ACPI Component Architecture
 * AML/ASL+ Disassembler version 20200925 (64-bit version)
 * Copyright (c) 2000 - 2020 Intel Corporation
 * 
 * Disassembling to symbolic ASL+ operators
 *
 * Disassembly of vmware.dsdt.aml, Fri Jul 12 14:58:57 2024
 *
 * Original Table Header:
 *     Signature        "DSDT"
 *     Length           0x0000A35B (41819)
 *     Revision         0x01 **** 32-bit table (V1), no 64-bit math support
 *     Checksum         0x41
 *     OEM ID           "PTLTD "
 *     OEM Table ID     "Custom  "
 *     OEM Revision     0x00000000 (0)
 *     Compiler ID      "INTL"
 *     Compiler Version 0x20130823 (538118179)
 */
DefinitionBlock ("", "DSDT", 1, "PTLTD ", "Custom  ", 0x00000000)
{
    Name (GPIC, Zero)
    Method (_PIC, 1, NotSerialized)  // _PIC: Interrupt Model
    {
        GPIC = Arg0
    }

    Scope (_SB)
    {
        Method (STRC, 2, NotSerialized)
        {
            If ((SizeOf (Arg0) != SizeOf (Arg1)))
            {
                Return (Zero)
            }

            Local0 = (SizeOf (Arg0) + One)
            Name (BUF0, Buffer (Local0) {})
            Name (BUF1, Buffer (Local0) {})
            BUF0 = Arg0
            BUF1 = Arg1
            While (Local0)
            {
                Local0--
                If ((DerefOf (BUF0 [Local0]) != DerefOf (BUF1 [Local0]
                    )))
                {
                    Return (Zero)
                }
            }

            Return (One)
        }

        OperationRegion (OEMD, SystemMemory, 0x0FF72040, 0x00000068)
        Field (OEMD, AnyAcc, NoLock, Preserve)
        {
            Offset (0x24), 
            CCAP,   32, 
            ECFG,   32, 
            VMGC,   256, 
            LDDV,   32, 
            POSC,   32, 
            SIOC,   32, 
            NVDM,   32, 
            NVDH,   16, 
            Offset (0x60), 
            MSHD,   32, 
            ECFP,   32
        }

        Name (TOOS, Zero)
        Name (FLAG, Zero)
        Method (_INI, 0, NotSerialized)  // _INI: Initialize
        {
            If ((FLAG != Zero))
            {
                Return (Zero)
            }

            FLAG = One
            If (CondRefOf (\_OSI, Local0))
            {
                If (_OSI ("Linux"))
                {
                    TOOS = 0x0F00
                }
                ElseIf (_OSI ("Darwin"))
                {
                    TOOS = 0x0D00
                }
                ElseIf (_OSI ("Windows 2006"))
                {
                    TOOS = 0x0C00
                }
                ElseIf (_OSI ("Windows 2001.1 SP1"))
                {
                    TOOS = 0x0A00
                }
                ElseIf (_OSI ("Windows 2001.1"))
                {
                    TOOS = 0x0900
                }
                ElseIf (_OSI ("Windows 2001 SP2"))
                {
                    TOOS = 0x0700
                }
                ElseIf (_OSI ("Windows 2001 SP1"))
                {
                    TOOS = 0x0600
                }
                ElseIf (_OSI ("Windows 2001"))
                {
                    TOOS = 0x0500
                }
                ElseIf (_OSI ("Windows 2000"))
                {
                    TOOS = 0x0400
                }
                Else
                {
                    TOOS = 0xFE00
                }
            }
            ElseIf (CondRefOf (\_OS, Local0))
            {
                If (STRC (_OS, "Microsoft Windows NT"))
                {
                    TOOS = 0x0300
                }
                ElseIf (STRC (_OS, "Microsoft WindowsME:Millennium Edition"))
                {
                    TOOS = 0x0200
                }
                ElseIf (STRC (_OS, "Microsoft Windows"))
                {
                    TOOS = 0x0100
                }
                ElseIf (STRC (_OS, "NetWare"))
                {
                    TOOS = 0x0B00
                }
                ElseIf (STRC (_OS, "FreeBSD"))
                {
                    TOOS = 0x1000
                }
                Else
                {
                    TOOS = 0xFD00
                }
            }
            Else
            {
                TOOS = 0xFF00
            }

            Return (Zero)
        }

        Method (PPRT, 0, NotSerialized)
        {
            If ((GPIC == One))
            {
                Return (Package (0x80)
                {
                    Package (0x04)
                    {
                        0xFFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0xFFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0xFFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0xFFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        Zero, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        One, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        0x02, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        0x03, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        Zero, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        One, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        0x02, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        0x03, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        Zero, 
                        Zero, 
                        0x13
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        One, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        0x02, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        0x03, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        Zero, 
                        Zero, 
                        0x10
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        One, 
                        Zero, 
                        0x11
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        0x02, 
                        Zero, 
                        0x12
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        0x03, 
                        Zero, 
                        0x13
                    }
                })
            }
            Else
            {
                Return (Package (0x80)
                {
                    Package (0x04)
                    {
                        0xFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0xFFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0xFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0xFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0001FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0002FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0003FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0004FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0005FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0006FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0007FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0008FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0009FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000AFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000BFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000CFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000DFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000EFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x000FFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0010FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0011FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0012FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0013FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0014FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0015FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0016FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0017FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0018FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x0019FFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001AFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001BFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        One, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001CFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        One, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001DFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        One, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001EFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        Zero, 
                        ^PCI0.ISA.LNKA, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        One, 
                        ^PCI0.ISA.LNKB, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        0x02, 
                        ^PCI0.ISA.LNKC, 
                        Zero
                    }, 

                    Package (0x04)
                    {
                        0x001FFFFF, 
                        0x03, 
                        ^PCI0.ISA.LNKD, 
                        Zero
                    }
                })
            }
        }

        Mutex (OEML, 0x0F)
        Device (PCI0)
        {
            Name (_HID, EisaId ("PNP0A03") /* PCI Bus */)  // _HID: Hardware ID
            Name (_CID, EisaId ("PNP0A08") /* PCI Express Bus */)  // _CID: Compatible ID
            Method (_INI, 0, NotSerialized)  // _INI: Initialize
            {
                If ((TOOS == 0x0D00))
                {
                    _HID = 0x080AD041
                    _CID = 0x030AD041
                }
            }

            Name (_ADR, Zero)  // _ADR: Address
            OperationRegion (REGS, PCI_Config, 0x50, 0x30)
            Field (REGS, DWordAcc, NoLock, Preserve)
            {
                Offset (0x0A), 
                PMC0,   2, 
                    ,   2, 
                PMC4,   2, 
                Offset (0x0B), 
                PMC8,   2, 
                    ,   2, 
                PMCC,   2, 
                Offset (0x0C), 
                PMD0,   2, 
                    ,   2, 
                PMD4,   2, 
                Offset (0x0D), 
                PMD8,   2, 
                    ,   2, 
                PMDC,   2, 
                Offset (0x0E), 
                PME0,   2, 
                    ,   2, 
                PME4,   2, 
                Offset (0x0F), 
                PME8,   2, 
                    ,   2, 
                PMEC,   2, 
                Offset (0x10), 
                Offset (0x2A), 
                CRST,   1
            }

            OperationRegion (RE00, PCI_Config, 0xD8, 0x04)
            Field (RE00, DWordAcc, NoLock, Preserve)
            {
                OEMR,   32
            }

            Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
            {
                Local0 = ResourceTemplate ()
                    {
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000A0000,         // Range Minimum
                            0x000BFFFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00020000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000C0000,         // Range Minimum
                            0x000C3FFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000C4000,         // Range Minimum
                            0x000C7FFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000C8000,         // Range Minimum
                            0x000CBFFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000CC000,         // Range Minimum
                            0x000CFFFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000D0000,         // Range Minimum
                            0x000D3FFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000D4000,         // Range Minimum
                            0x000D7FFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000D8000,         // Range Minimum
                            0x000DBFFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000DC000,         // Range Minimum
                            0x000DFFFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000E0000,         // Range Minimum
                            0x000E3FFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000E4000,         // Range Minimum
                            0x000E7FFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000E8000,         // Range Minimum
                            0x000EBFFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        DWordMemory (ResourceProducer, PosDecode, MinFixed, MaxFixed, Cacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x000EC000,         // Range Minimum
                            0x000EFFFF,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00004000,         // Length
                            0x00,, , AddressRangeMemory, TypeStatic)
                        IO (Decode16,
                            0x0CF8,             // Range Minimum
                            0x0CF8,             // Range Maximum
                            0x01,               // Alignment
                            0x08,               // Length
                            )
                    }
                If (PMC0)
                {
                    CreateDWordField (Local0, 0x31, C0LN)
                    C0LN = Zero
                }

                If (PMC4)
                {
                    CreateDWordField (Local0, 0x4C, C4LN)
                    C4LN = Zero
                }

                If (PMC8)
                {
                    CreateDWordField (Local0, 0x67, C8LN)
                    C8LN = Zero
                }

                If (PMCC)
                {
                    CreateDWordField (Local0, 0x82, CCLN)
                    CCLN = Zero
                }

                If (PMD0)
                {
                    CreateDWordField (Local0, 0x9D, D0LN)
                    D0LN = Zero
                }

                If (PMD4)
                {
                    CreateDWordField (Local0, 0xB8, D4LN)
                    D4LN = Zero
                }

                If (PMD8)
                {
                    CreateDWordField (Local0, 0xD3, D8LN)
                    D8LN = Zero
                }

                If (PMDC)
                {
                    CreateDWordField (Local0, 0xEE, DCLN)
                    DCLN = Zero
                }

                If (PME0)
                {
                    CreateDWordField (Local0, 0x0109, E0LN)
                    E0LN = Zero
                }

                If (PME4)
                {
                    CreateDWordField (Local0, 0x0124, E4LN)
                    E4LN = Zero
                }

                If (PME8)
                {
                    CreateDWordField (Local0, 0x013F, E8LN)
                    E8LN = Zero
                }

                If (PMEC)
                {
                    CreateDWordField (Local0, 0x015A, ECLN)
                    ECLN = Zero
                }

                Return (Concatenate (XCRS, Local0))
            }

            Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
            {
                0x03, 
                0x03
            })
            Method (_PRT, 0, NotSerialized)  // _PRT: PCI Routing Table
            {
                Return (PPRT ())
            }

            Device (AGP)
            {
                Name (_ADR, 0x00010000)  // _ADR: Address
            }

            Device (ISA)
            {
                Name (_ADR, 0x00070000)  // _ADR: Address
                Device (MBRD)
                {
                    Name (_HID, EisaId ("PNP0C02") /* PNP Motherboard Resources */)  // _HID: Hardware ID
                    Name (_UID, 0x1F)  // _UID: Unique ID
                    Name (RSRC, ResourceTemplate ()
                    {
                        IO (Decode16,
                            0x0010,             // Range Minimum
                            0x0010,             // Range Maximum
                            0x01,               // Alignment
                            0x10,               // Length
                            )
                        IO (Decode16,
                            0x0024,             // Range Minimum
                            0x0024,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x0028,             // Range Minimum
                            0x0028,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x002C,             // Range Minimum
                            0x002C,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x002E,             // Range Minimum
                            0x002E,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x0030,             // Range Minimum
                            0x0030,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x0034,             // Range Minimum
                            0x0034,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x0038,             // Range Minimum
                            0x0038,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x003C,             // Range Minimum
                            0x003C,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x0050,             // Range Minimum
                            0x0050,             // Range Maximum
                            0x01,               // Alignment
                            0x04,               // Length
                            )
                        IO (Decode16,
                            0x0072,             // Range Minimum
                            0x0072,             // Range Maximum
                            0x01,               // Alignment
                            0x06,               // Length
                            )
                        IO (Decode16,
                            0x0080,             // Range Minimum
                            0x0080,             // Range Maximum
                            0x01,               // Alignment
                            0x01,               // Length
                            )
                        IO (Decode16,
                            0x0090,             // Range Minimum
                            0x0090,             // Range Maximum
                            0x01,               // Alignment
                            0x10,               // Length
                            )
                        IO (Decode16,
                            0x00A4,             // Range Minimum
                            0x00A4,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x00A8,             // Range Minimum
                            0x00A8,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x00AC,             // Range Minimum
                            0x00AC,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x00B0,             // Range Minimum
                            0x00B0,             // Range Maximum
                            0x01,               // Alignment
                            0x06,               // Length
                            )
                        IO (Decode16,
                            0x00B8,             // Range Minimum
                            0x00B8,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x00BC,             // Range Minimum
                            0x00BC,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x0100,             // Range Minimum
                            0x0100,             // Range Maximum
                            0x01,               // Alignment
                            0x40,               // Length
                            _Y00)
                        IO (Decode16,
                            0x0140,             // Range Minimum
                            0x0140,             // Range Maximum
                            0x01,               // Alignment
                            0x10,               // Length
                            _Y01)
                        IO (Decode16,
                            0x5658,             // Range Minimum
                            0x5658,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x5670,             // Range Minimum
                            0x5670,             // Range Maximum
                            0x01,               // Alignment
                            0x01,               // Length
                            )
                        IO (Decode16,
                            0x0CF0,             // Range Minimum
                            0x0CF0,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                    })
                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        CreateWordField (RSRC, \_SB.PCI0.ISA.MBRD._Y00._MIN, PMMN)  // _MIN: Minimum Base Address
                        CreateWordField (RSRC, \_SB.PCI0.ISA.MBRD._Y00._MAX, PMMX)  // _MAX: Maximum Base Address
                        PMMN = (^^^PWR.PMBA & 0xFFFFFFFE)
                        PMMX = PMMN /* \_SB_.PCI0.ISA_.MBRD._CRS.PMMN */
                        CreateWordField (RSRC, \_SB.PCI0.ISA.MBRD._Y01._MIN, SMMN)  // _MIN: Minimum Base Address
                        CreateWordField (RSRC, \_SB.PCI0.ISA.MBRD._Y01._MAX, SMMX)  // _MAX: Maximum Base Address
                        SMMN = (^^^PWR.SBBA & 0xFFFFFFFE)
                        SMMX = SMMN /* \_SB_.PCI0.ISA_.MBRD._CRS.SMMN */
                        Return (RSRC) /* \_SB_.PCI0.ISA_.MBRD.RSRC */
                    }
                }

                Device (DMAC)
                {
                    Name (_HID, EisaId ("PNP0200") /* PC-class DMA Controller */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0000,             // Range Minimum
                            0x0000,             // Range Maximum
                            0x01,               // Alignment
                            0x10,               // Length
                            )
                        IO (Decode16,
                            0x0081,             // Range Minimum
                            0x0081,             // Range Maximum
                            0x01,               // Alignment
                            0x0F,               // Length
                            )
                        IO (Decode16,
                            0x00C0,             // Range Minimum
                            0x00C0,             // Range Maximum
                            0x01,               // Alignment
                            0x20,               // Length
                            )
                        DMA (Compatibility, NotBusMaster, Transfer16, )
                            {4}
                    })
                }

                Device (PIC)
                {
                    Name (_HID, EisaId ("PNP0001") /* EISA Interrupt Controller */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0020,             // Range Minimum
                            0x0020,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x00A0,             // Range Minimum
                            0x00A0,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IO (Decode16,
                            0x04D0,             // Range Minimum
                            0x04D0,             // Range Maximum
                            0x01,               // Alignment
                            0x02,               // Length
                            )
                        IRQ (Edge, ActiveHigh, Exclusive, )
                            {2}
                    })
                }

                Device (TIME)
                {
                    Name (_HID, EisaId ("PNP0100") /* PC-class System Timer */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0040,             // Range Minimum
                            0x0040,             // Range Maximum
                            0x01,               // Alignment
                            0x04,               // Length
                            )
                        IRQNoFlags ()
                            {0}
                    })
                }

                Device (RTC)
                {
                    Name (_HID, EisaId ("PNP0B00") /* AT Real-Time Clock */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0070,             // Range Minimum
                            0x0070,             // Range Maximum
                            0x01,               // Alignment
                            0x04,               // Length
                            )
                        IRQNoFlags ()
                            {8}
                    })
                }

                Device (SPKR)
                {
                    Name (_HID, EisaId ("PNP0800") /* Microsoft Sound System Compatible Device */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0061,             // Range Minimum
                            0x0061,             // Range Maximum
                            0x01,               // Alignment
                            0x01,               // Length
                            )
                    })
                }

                Device (KBC)
                {
                    Name (_HID, EisaId ("PNP0303") /* IBM Enhanced Keyboard (101/102-key, PS/2 Mouse) */)  // _HID: Hardware ID
                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IO (Decode16,
                            0x0060,             // Range Minimum
                            0x0060,             // Range Maximum
                            0x01,               // Alignment
                            0x01,               // Length
                            )
                        IO (Decode16,
                            0x0064,             // Range Minimum
                            0x0064,             // Range Maximum
                            0x01,               // Alignment
                            0x01,               // Length
                            )
                        IRQNoFlags ()
                            {1}
                    })
                }

                Device (MOUS)
                {
                    Method (_HID, 0, NotSerialized)  // _HID: Hardware ID
                    {
                        Return (MSHD) /* \_SB_.MSHD */
                    }

                    Method (_CID, 0, NotSerialized)  // _CID: Compatible ID
                    {
                        Local0 = 0x130FD041
                        If ((MSHD == Local0))
                        {
                            Local0 = 0x0300B759
                        }

                        Return (Local0)
                    }

                    Name (_CRS, ResourceTemplate ()  // _CRS: Current Resource Settings
                    {
                        IRQNoFlags ()
                            {12}
                    })
                }

                Device (LNKA)
                {
                    Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
                    Name (_UID, One)  // _UID: Unique ID
                    Name (_PRS, ResourceTemplate ()  // _PRS: Possible Resource Settings
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {3,4,5,6,7,9,10,11,14,15}
                    })
                    Name (RSRC, ResourceTemplate ()
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {}
                    })
                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        PIRA |= 0x80
                    }

                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        CreateWordField (RSRC, One, IRQ0)
                        Local0 = (PIRA & 0x0F)
                        IRQ0 = (One << Local0)
                        Return (RSRC) /* \_SB_.PCI0.ISA_.LNKA.RSRC */
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, One, IRQ0)
                        FindSetRightBit (IRQ0, Local0)
                        Local0--
                        PIRA = (Local0 | (PIRA & 0x70))
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If ((PIRA & 0x80))
                        {
                            Return (0x09)
                        }

                        Return (0x0B)
                    }
                }

                Device (LNKB)
                {
                    Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
                    Name (_UID, 0x02)  // _UID: Unique ID
                    Name (_PRS, ResourceTemplate ()  // _PRS: Possible Resource Settings
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {3,4,5,6,7,9,10,11,14,15}
                    })
                    Name (RSRC, ResourceTemplate ()
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {}
                    })
                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        PIRB |= 0x80
                    }

                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        CreateWordField (RSRC, One, IRQ0)
                        Local0 = (PIRB & 0x0F)
                        IRQ0 = (One << Local0)
                        Return (RSRC) /* \_SB_.PCI0.ISA_.LNKB.RSRC */
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, One, IRQ0)
                        FindSetRightBit (IRQ0, Local0)
                        Local0--
                        PIRB = (Local0 | (PIRB & 0x70))
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If ((PIRB & 0x80))
                        {
                            Return (0x09)
                        }

                        Return (0x0B)
                    }
                }

                Device (LNKC)
                {
                    Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
                    Name (_UID, 0x03)  // _UID: Unique ID
                    Name (_PRS, ResourceTemplate ()  // _PRS: Possible Resource Settings
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {3,4,5,6,7,9,10,11,14,15}
                    })
                    Name (RSRC, ResourceTemplate ()
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {}
                    })
                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        PIRC |= 0x80
                    }

                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        CreateWordField (RSRC, One, IRQ0)
                        Local0 = (PIRC & 0x0F)
                        IRQ0 = (One << Local0)
                        Return (RSRC) /* \_SB_.PCI0.ISA_.LNKC.RSRC */
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, One, IRQ0)
                        FindSetRightBit (IRQ0, Local0)
                        Local0--
                        PIRC = (Local0 | (PIRC & 0x70))
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If ((PIRC & 0x80))
                        {
                            Return (0x09)
                        }

                        Return (0x0B)
                    }
                }

                Device (LNKD)
                {
                    Name (_HID, EisaId ("PNP0C0F") /* PCI Interrupt Link Device */)  // _HID: Hardware ID
                    Name (_UID, 0x04)  // _UID: Unique ID
                    Name (_PRS, ResourceTemplate ()  // _PRS: Possible Resource Settings
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {3,4,5,6,7,9,10,11,14,15}
                    })
                    Name (RSRC, ResourceTemplate ()
                    {
                        IRQ (Level, ActiveLow, Shared, )
                            {}
                    })
                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        PIRD |= 0x80
                    }

                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        CreateWordField (RSRC, One, IRQ0)
                        Local0 = (PIRD & 0x0F)
                        IRQ0 = (One << Local0)
                        Return (RSRC) /* \_SB_.PCI0.ISA_.LNKD.RSRC */
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, One, IRQ0)
                        FindSetRightBit (IRQ0, Local0)
                        Local0--
                        PIRD = (Local0 | (PIRD & 0x70))
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If ((PIRD & 0x80))
                        {
                            Return (0x09)
                        }

                        Return (0x0B)
                    }
                }

                OperationRegion (PIRX, PCI_Config, 0x60, 0x04)
                Field (PIRX, DWordAcc, NoLock, Preserve)
                {
                    AccessAs (ByteAcc, 0x00), 
                    PIRA,   8, 
                    PIRB,   8, 
                    PIRC,   8, 
                    PIRD,   8
                }

                OperationRegion (LPCS, SystemMemory, ECFG, 0x0500)
                Device (HPET)
                {
                    Name (_HID, EisaId ("PNP0103") /* HPET System Timer */)  // _HID: Hardware ID
                    Name (_CID, EisaId ("PNP0C01") /* System Board */)  // _CID: Compatible ID
                    OperationRegion (EICH, SystemMemory, (ECFG + 0x4000), 0x4000)
                    Field (EICH, DWordAcc, Lock, Preserve)
                    {
                        Offset (0x3404), 
                        AS,     2, 
                            ,   5, 
                        AE,     1
                    }

                    Field (LPCS, DWordAcc, Lock, Preserve)
                    {
                        Offset (0x328), 
                        HBAS,   32
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If ((AE | HBAS))
                        {
                            Return (0x0F)
                        }

                        Return (Zero)
                    }

                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        Name (TMPL, ResourceTemplate ()
                        {
                            Memory32Fixed (ReadWrite,
                                0xFED00000,         // Address Base
                                0x00000400,         // Address Length
                                _Y02)
                        })
                        CreateDWordField (TMPL, \_SB.PCI0.ISA.HPET._CRS._Y02._BAS, BASE)  // _BAS: Base Address
                        If (HBAS)
                        {
                            BASE = HBAS /* \_SB_.PCI0.ISA_.HPET.HBAS */
                        }

                        Return (TMPL) /* \_SB_.PCI0.ISA_.HPET._CRS.TMPL */
                    }
                }

                Device (SMC)
                {
                    Name (_HID, EisaId ("APP0001"))  // _HID: Hardware ID
                    Name (_CID, "smc-santarosa")  // _CID: Compatible ID
                    Field (LPCS, DWordAcc, NoLock, Preserve)
                    {
                        Offset (0x200), 
                        SMPR,   8, 
                        SMEN,   8, 
                        SMIR,   8, 
                        Offset (0x208), 
                        SMIO,   16, 
                        Offset (0x20C), 
                        SMVR,   8, 
                        Offset (0x210), 
                        SMMM,   32
                    }

                    Method (_INI, 0, NotSerialized)  // _INI: Initialize
                    {
                        If (SMVR)
                        {
                            _CID = "smc-huronriver"
                        }
                    }

                    Method (_STA, 0, Serialized)  // _STA: Status
                    {
                        If (SMEN)
                        {
                            Return (0x0B)
                        }

                        Return (Zero)
                    }

                    Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                    {
                        Local0 = ResourceTemplate ()
                            {
                                IO (Decode16,
                                    0x0000,             // Range Minimum
                                    0x0000,             // Range Maximum
                                    0x01,               // Alignment
                                    0x20,               // Length
                                    )
                                IRQNoFlags ()
                                    {}
                            }
                        CreateWordField (Local0, 0x02, IOMN)
                        CreateWordField (Local0, 0x04, IOMX)
                        CreateWordField (Local0, 0x09, IRQS)
                        IOMN = SMIO /* \_SB_.PCI0.ISA_.SMC_.SMIO */
                        IOMX = SMIO /* \_SB_.PCI0.ISA_.SMC_.SMIO */
                        IRQS = (One << SMIR)
                        If (SMVR)
                        {
                            Local1 = Buffer (0x0C)
                                {
                                    /* 0000 */  0x86, 0x09, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,  // ........
                                    /* 0008 */  0x00, 0x00, 0x01, 0x00                           // ....
                                }
                            CreateDWordField (Local1, 0x04, MMBS)
                            MMBS = SMMM /* \_SB_.PCI0.ISA_.SMC_.SMMM */
                            Return (Concatenate (Local1, Local0))
                        }

                        Return (Local0)
                    }
                }

                Name (RSR2, ResourceTemplate ()
                {
                    IO (Decode16,
                        0x0000,             // Range Minimum
                        0x0000,             // Range Maximum
                        0x04,               // Alignment
                        0x04,               // Length
                        )
                    IRQNoFlags ()
                        {}
                })
                Device (COM3)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x03)  // _UID: Unique ID
                    Name (_DDN, "COM3")  // _DDN: DOS Device Name
                    Field (LPCS, ByteAcc, NoLock, Preserve)
                    {
                        Offset (0x2A0), 
                        PRES,   8, 
                        ENAB,   8, 
                        IRQL,   8, 
                        Offset (0x2A8), 
                        AccessAs (WordAcc, 0x00), 
                        IOBA,   16, 
                        AccessAs (ByteAcc, 0x00), 
                        Offset (0x480), 
                            ,   7, 
                        VALD,   1
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If (!VALD)
                        {
                            If (PRES)
                            {
                                If (ENAB)
                                {
                                    Return (0x0F)
                                }

                                Return (0x0D)
                            }
                        }

                        Return (Zero)
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, 0x09, IRQW)
                        If (IRQW)
                        {
                            FindSetRightBit (IRQW, Local0)
                            IRQL = (Local0 - One)
                        }
                        Else
                        {
                            IRQL = 0xFF
                        }

                        CreateWordField (Arg0, 0x02, IOAL)
                        IOBA = IOAL /* \_SB_.PCI0.ISA_.COM3._SRS.IOAL */
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        ENAB = Zero
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (LCRS (IOBA, IRQL, One))
                    }

                    Name (_PRS, ResourceTemplate ()  // _PRS: Possible Resource Settings
                    {
                        StartDependentFnNoPri ()
                        {
                            IO (Decode16,
                                0x03E8,             // Range Minimum
                                0x03E8,             // Range Maximum
                                0x08,               // Alignment
                                0x08,               // Length
                                )
                            IRQNoFlags ()
                                {3,4,5,6,7,9,10,11,12,14,15}
                        }
                        StartDependentFnNoPri ()
                        {
                            IO (Decode16,
                                0x02E8,             // Range Minimum
                                0x02E8,             // Range Maximum
                                0x08,               // Alignment
                                0x08,               // Length
                                )
                            IRQNoFlags ()
                                {3,4,5,6,7,9,10,11,12,14,15}
                        }
                        StartDependentFn (0x02, 0x01)
                        {
                            IO (Decode16,
                                0x0100,             // Range Minimum
                                0x03F8,             // Range Maximum
                                0x08,               // Alignment
                                0x08,               // Length
                                )
                            IRQNoFlags ()
                                {3,4,5,6,7,9,10,11,12,14,15}
                        }
                        EndDependentFn ()
                    })
                }

                Device (COM4)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x04)  // _UID: Unique ID
                    Name (_DDN, "COM4")  // _DDN: DOS Device Name
                    Field (LPCS, ByteAcc, NoLock, Preserve)
                    {
                        Offset (0x2C0), 
                        PRES,   8, 
                        ENAB,   8, 
                        IRQL,   8, 
                        Offset (0x2C8), 
                        AccessAs (WordAcc, 0x00), 
                        IOBA,   16, 
                        AccessAs (ByteAcc, 0x00), 
                        Offset (0x480), 
                            ,   7, 
                        VALD,   1
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If (!VALD)
                        {
                            If (PRES)
                            {
                                If (ENAB)
                                {
                                    Return (0x0F)
                                }

                                Return (0x0D)
                            }
                        }

                        Return (Zero)
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, 0x09, IRQW)
                        If (IRQW)
                        {
                            FindSetRightBit (IRQW, Local0)
                            IRQL = (Local0 - One)
                        }
                        Else
                        {
                            IRQL = 0xFF
                        }

                        CreateWordField (Arg0, 0x02, IOAL)
                        IOBA = IOAL /* \_SB_.PCI0.ISA_.COM4._SRS.IOAL */
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        ENAB = Zero
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (LCRS (IOBA, IRQL, One))
                    }

                    Name (_PRS, ResourceTemplate ()  // _PRS: Possible Resource Settings
                    {
                        StartDependentFnNoPri ()
                        {
                            IO (Decode16,
                                0x02E8,             // Range Minimum
                                0x02E8,             // Range Maximum
                                0x08,               // Alignment
                                0x08,               // Length
                                )
                            IRQNoFlags ()
                                {3,4,5,6,7,9,10,11,12,14,15}
                        }
                        StartDependentFnNoPri ()
                        {
                            IO (Decode16,
                                0x03E8,             // Range Minimum
                                0x03E8,             // Range Maximum
                                0x08,               // Alignment
                                0x08,               // Length
                                )
                            IRQNoFlags ()
                                {3,4,5,6,7,9,10,11,12,14,15}
                        }
                        StartDependentFn (0x02, 0x01)
                        {
                            IO (Decode16,
                                0x0100,             // Range Minimum
                                0x03F8,             // Range Maximum
                                0x08,               // Alignment
                                0x08,               // Length
                                )
                            IRQNoFlags ()
                                {3,4,5,6,7,9,10,11,12,14,15}
                        }
                        EndDependentFn ()
                    })
                }

                Device (LP02)
                {
                    Name (_HID, EisaId ("PNP0400") /* Standard LPT Parallel Port */)  // _HID: Hardware ID
                    Name (_UID, 0x02)  // _UID: Unique ID
                    Name (_DDN, "LPT2")  // _DDN: DOS Device Name
                    Field (LPCS, ByteAcc, NoLock, Preserve)
                    {
                        Offset (0x2E0), 
                        PRES,   8, 
                        ENAB,   8, 
                        IRQL,   8, 
                        Offset (0x2E8), 
                        AccessAs (WordAcc, 0x00), 
                        IOBA,   16, 
                        Offset (0x2EC), 
                        AccessAs (DWordAcc, 0x00), 
                        XTRA,   32
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If (PRES)
                        {
                            If (ENAB)
                            {
                                Return (0x0F)
                            }

                            Return (0x0D)
                        }

                        Return (Zero)
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, 0x09, IRQW)
                        If (IRQW)
                        {
                            FindSetRightBit (IRQW, Local0)
                            IRQL = (Local0 - One)
                        }
                        Else
                        {
                            IRQL = 0xFF
                        }

                        CreateWordField (Arg0, 0x02, IOAL)
                        IOBA = IOAL /* \_SB_.PCI0.ISA_.LP02._SRS.IOAL */
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        ENAB = Zero
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (LCRS (IOBA, IRQL, (XTRA == 0x03)))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (LPRS ((XTRA == 0x03), 0x0278, 0x07, 0xDEF8))
                    }
                }

                Device (LPT3)
                {
                    Name (_HID, EisaId ("PNP0400") /* Standard LPT Parallel Port */)  // _HID: Hardware ID
                    Name (_UID, 0x03)  // _UID: Unique ID
                    Name (_DDN, "LPT3")  // _DDN: DOS Device Name
                    Field (LPCS, ByteAcc, NoLock, Preserve)
                    {
                        Offset (0x300), 
                        PRES,   8, 
                        ENAB,   8, 
                        IRQL,   8, 
                        Offset (0x308), 
                        AccessAs (WordAcc, 0x00), 
                        IOBA,   16, 
                        Offset (0x30C), 
                        AccessAs (DWordAcc, 0x00), 
                        XTRA,   32
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        If (PRES)
                        {
                            If (ENAB)
                            {
                                Return (0x0F)
                            }

                            Return (0x0D)
                        }

                        Return (Zero)
                    }

                    Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                    {
                        CreateWordField (Arg0, 0x09, IRQW)
                        If (IRQW)
                        {
                            FindSetRightBit (IRQW, Local0)
                            IRQL = (Local0 - One)
                        }
                        Else
                        {
                            IRQL = 0xFF
                        }

                        CreateWordField (Arg0, 0x02, IOAL)
                        IOBA = IOAL /* \_SB_.PCI0.ISA_.LPT3._SRS.IOAL */
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        ENAB = Zero
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (LCRS (IOBA, IRQL, (XTRA == 0x03)))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Local0 = (XTRA == 0x03)
                        Local1 = 0x03BC
                        If (Local0)
                        {
                            Local1 = 0x0378
                        }

                        Return (LPRS (Local0, Local1, Zero, 0xDEF8))
                    }
                }

                Name (SRSR, ResourceTemplate ()
                {
                    IO (Decode16,
                        0x0000,             // Range Minimum
                        0x0000,             // Range Maximum
                        0x08,               // Alignment
                        0x08,               // Length
                        )
                    IRQ (Level, ActiveLow, Shared, )
                        {0}
                })
                Method (SPRS, 0, Serialized)
                {
                    Local0 = ResourceTemplate ()
                        {
                            StartDependentFn (0x01, 0x00)
                            {
                                IO (Decode16,
                                    0x0100,             // Range Minimum
                                    0xFEF8,             // Range Maximum
                                    0x08,               // Alignment
                                    0x08,               // Length
                                    )
                                IRQ (Level, ActiveLow, Shared, )
                                    {3,4,5,6,7,9,10,11,12,14,15}
                            }
                            EndDependentFn ()
                        }
                    Return (Local0)
                }

                Method (SCOM, 3, Serialized)
                {
                    OperationRegion (SPRT, SystemMemory, (ECFG + Arg1), 0x04)
                    Field (SPRT, ByteAcc, NoLock, Preserve)
                    {
                        STAV,   4, 
                            ,   3, 
                        VALD,   1, 
                        IRQL,   8, 
                        AccessAs (WordAcc, 0x00), 
                        IOBA,   16
                    }

                    If (!Arg0)
                    {
                        Return (STAV) /* \_SB_.PCI0.ISA_.SCOM.STAV */
                    }

                    If ((Arg0 == One))
                    {
                        STAV = 0x0D
                        Return (Zero)
                    }

                    If ((Arg0 == 0x02))
                    {
                        Local0 = SRSR /* \_SB_.PCI0.ISA_.SRSR */
                    }
                    Else
                    {
                        Local0 = Arg2
                    }

                    CreateWordField (Local0, 0x02, IOAL)
                    CreateWordField (Local0, 0x04, IOAH)
                    CreateWordField (Local0, 0x09, IRQW)
                    If ((Arg0 == 0x02))
                    {
                        Local1 = IOBA /* \_SB_.PCI0.ISA_.SCOM.IOBA */
                        IOAL = Local1
                        IOAH = Local1
                        Local1 = IRQL /* \_SB_.PCI0.ISA_.SCOM.IRQL */
                        If ((Local1 == 0xFF))
                        {
                            IRQW = Zero
                        }
                        Else
                        {
                            IRQW = (One << Local1)
                        }

                        Return (Local0)
                    }

                    If (IRQW)
                    {
                        FindSetRightBit (IRQW, Local1)
                        IRQL = (Local1 - One)
                    }
                    Else
                    {
                        IRQL = 0xFF
                    }

                    IOBA = IOAL /* \_SB_.PCI0.ISA_.SCOM.IOAL */
                    Return (Zero)
                }

                Method (SSTA, 1, NotSerialized)
                {
                    Return (SCOM (Zero, Arg0, Zero))
                }

                Method (SDIS, 1, NotSerialized)
                {
                    SCOM (One, Arg0, Zero)
                }

                Method (SCRS, 1, NotSerialized)
                {
                    Return (SCOM (0x02, Arg0, Zero))
                }

                Method (SSRS, 2, NotSerialized)
                {
                    SCOM (0x03, Arg0, Arg1)
                }

                Device (CO02)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x03)  // _UID: Unique ID
                    Name (_DDN, "COM3")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x0488))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x0488)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x0488, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x0488))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO03)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x04)  // _UID: Unique ID
                    Name (_DDN, "COM4")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x048C))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x048C)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x048C, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x048C))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO04)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x05)  // _UID: Unique ID
                    Name (_DDN, "COM5")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x0490))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x0490)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x0490, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x0490))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO05)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x06)  // _UID: Unique ID
                    Name (_DDN, "COM6")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x0494))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x0494)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x0494, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x0494))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO06)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x07)  // _UID: Unique ID
                    Name (_DDN, "COM7")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x0498))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x0498)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x0498, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x0498))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO07)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x08)  // _UID: Unique ID
                    Name (_DDN, "COM8")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x049C))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x049C)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x049C, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x049C))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO08)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x09)  // _UID: Unique ID
                    Name (_DDN, "COM9")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04A0))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04A0)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04A0, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04A0))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO09)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x0A)  // _UID: Unique ID
                    Name (_DDN, "COM10")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04A4))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04A4)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04A4, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04A4))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO0A)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x0B)  // _UID: Unique ID
                    Name (_DDN, "COM11")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04A8))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04A8)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04A8, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04A8))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO0B)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x0C)  // _UID: Unique ID
                    Name (_DDN, "COM12")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04AC))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04AC)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04AC, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04AC))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO0C)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x0D)  // _UID: Unique ID
                    Name (_DDN, "COM13")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04B0))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04B0)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04B0, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04B0))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO0D)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x0E)  // _UID: Unique ID
                    Name (_DDN, "COM14")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04B4))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04B4)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04B4, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04B4))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO0E)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x0F)  // _UID: Unique ID
                    Name (_DDN, "COM15")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04B8))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04B8)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04B8, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04B8))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO0F)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x10)  // _UID: Unique ID
                    Name (_DDN, "COM16")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04BC))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04BC)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04BC, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04BC))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO10)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x11)  // _UID: Unique ID
                    Name (_DDN, "COM17")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04C0))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04C0)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04C0, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04C0))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO11)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x12)  // _UID: Unique ID
                    Name (_DDN, "COM18")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04C4))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04C4)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04C4, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04C4))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO12)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x13)  // _UID: Unique ID
                    Name (_DDN, "COM19")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04C8))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04C8)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04C8, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04C8))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO13)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x14)  // _UID: Unique ID
                    Name (_DDN, "COM20")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04CC))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04CC)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04CC, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04CC))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO14)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x15)  // _UID: Unique ID
                    Name (_DDN, "COM21")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04D0))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04D0)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04D0, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04D0))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO15)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x16)  // _UID: Unique ID
                    Name (_DDN, "COM22")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04D4))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04D4)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04D4, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04D4))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO16)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x17)  // _UID: Unique ID
                    Name (_DDN, "COM23")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04D8))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04D8)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04D8, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04D8))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO17)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x18)  // _UID: Unique ID
                    Name (_DDN, "COM24")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04DC))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04DC)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04DC, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04DC))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO18)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x19)  // _UID: Unique ID
                    Name (_DDN, "COM25")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04E0))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04E0)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04E0, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04E0))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO19)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x1A)  // _UID: Unique ID
                    Name (_DDN, "COM26")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04E4))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04E4)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04E4, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04E4))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO1A)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x1B)  // _UID: Unique ID
                    Name (_DDN, "COM27")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04E8))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04E8)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04E8, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04E8))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO1B)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x1C)  // _UID: Unique ID
                    Name (_DDN, "COM28")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04EC))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04EC)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04EC, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04EC))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO1C)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x1D)  // _UID: Unique ID
                    Name (_DDN, "COM29")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04F0))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04F0)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04F0, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04F0))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO1D)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x1E)  // _UID: Unique ID
                    Name (_DDN, "COM30")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04F4))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04F4)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04F4, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04F4))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO1E)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x1F)  // _UID: Unique ID
                    Name (_DDN, "COM31")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04F8))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04F8)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04F8, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04F8))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Device (CO1F)
                {
                    Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                    Name (_UID, 0x20)  // _UID: Unique ID
                    Name (_DDN, "COM32")  // _DDN: DOS Device Name
                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (SSTA (0x04FC))
                    }

                    Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                    {
                        SDIS (0x04FC)
                    }

                    Method (_SRS, 1, NotSerialized)  // _SRS: Set Resource Settings
                    {
                        SSRS (0x04FC, Arg0)
                    }

                    Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                    {
                        Return (SCRS (0x04FC))
                    }

                    Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                    {
                        Return (SPRS ())
                    }
                }

                Method (XRES, 6, Serialized)
                {
                    Local0 = Buffer (0x0D)
                        {
                            /* 0000 */  0x31, 0x00, 0x47, 0x01, 0x00, 0x00, 0x00, 0x00,  // 1.G.....
                            /* 0008 */  0x00, 0x00, 0x22, 0x00, 0x00                     // .."..
                        }
                    Local0 [One] = Arg0
                    CreateWordField (Local0, 0x04, IOAL)
                    IOAL = Arg1
                    CreateWordField (Local0, 0x06, IOAH)
                    IOAH = Arg2
                    Local0 [0x08] = Arg3
                    Local0 [0x09] = Arg3
                    CreateWordField (Local0, 0x0B, IRQF)
                    IRQF = Arg4
                    Return (Concatenate (Local0, Arg5))
                }

                Method (XPRS, 4, Serialized)
                {
                    Local1 = XRES (Arg0, 0x0100, 0x03F8, Arg1, Arg2, Arg3)
                    Local0 = (Arg0 - One)
                    If ((Arg1 == 0x04))
                    {
                        Local1 = XRES (Local0, 0x03BC, 0x03BC, Arg1, Arg2, Local1)
                    }

                    Local1 = XRES (Local0, 0x0278, 0x0278, Arg1, Arg2, Local1)
                    Local1 = XRES (Local0, 0x0378, 0x0378, Arg1, Arg2, Local1)
                    Return (Local1)
                }

                Method (LPRS, 4, Serialized)
                {
                    If (Arg0)
                    {
                        Local2 = 0x08
                    }
                    Else
                    {
                        Local2 = 0x04
                    }

                    Local1 = ResourceTemplate ()
                        {
                        }
                        EndDependentFn ()
                        }
                    Local1 = XPRS (0x0A, Local2, Zero, Local1)
                    Local1 = XPRS (0x06, Local2, Arg3, Local1)
                    Local3 = Zero
                    If (Arg2)
                    {
                        Local3 = (One << Arg2)
                    }

                    Local1 = XRES (One, Arg1, Arg1, Local2, Local3, Local1)
                    Return (Local1)
                }

                Name (RSRC, ResourceTemplate ()
                {
                    IO (Decode16,
                        0x0000,             // Range Minimum
                        0x0000,             // Range Maximum
                        0x04,               // Alignment
                        0x04,               // Length
                        )
                    IRQNoFlags ()
                        {}
                })
                Method (LCRS, 3, NotSerialized)
                {
                    Local1 = RSRC /* \_SB_.PCI0.ISA_.RSRC */
                    CreateWordField (Local1, 0x02, IOAL)
                    CreateWordField (Local1, 0x04, IOAH)
                    IOAL = Arg0
                    IOAH = Arg0
                    If ((Arg1 && (Arg1 != 0xFF)))
                    {
                        CreateWordField (Local1, 0x09, IRQW)
                        IRQW = (One << Arg1)
                    }

                    If (Arg2)
                    {
                        CreateByteField (Local1, 0x06, IOAN)
                        CreateByteField (Local1, 0x07, IOLN)
                        IOAN = 0x08
                        IOLN = 0x08
                    }

                    Return (Local1)
                }

                Device (SIO)
                {
                    Name (_HID, EisaId ("PNP0A05") /* Generic Container Device */)  // _HID: Hardware ID
                    OperationRegion (SIOR, SystemIO, 0x2E, 0x02)
                    Field (SIOR, ByteAcc, NoLock, Preserve)
                    {
                        SIOI,   8, 
                        SIOD,   8
                    }

                    IndexField (SIOI, SIOD, ByteAcc, NoLock, Preserve)
                    {
                        FLPT,   1, 
                        FCMA,   1, 
                        FCMB,   1, 
                        FDCA,   1, 
                        Offset (0x01), 
                        Offset (0x04), 
                        PEPP,   1, 
                        Offset (0x05), 
                        Offset (0x1B), 
                            ,   4, 
                        PPIR,   4, 
                        CAIR,   4, 
                        CBIR,   4, 
                        Offset (0x41), 
                        FCIR,   4, 
                        CDMA,   3, 
                        Offset (0x42), 
                        PBAL,   8, 
                        PBAH,   8, 
                        S1BL,   8, 
                        S1BH,   8, 
                        S2BL,   8, 
                        S2BH,   8, 
                        FBAL,   8, 
                        FBAH,   8
                    }

                    Method (SCRS, 4, NotSerialized)
                    {
                        Return (LCRS ((((Arg1 & 0xFC) << 0x08) | ((
                            Arg0 & 0xFF) << 0x02)), Arg2, Arg3))
                    }

                    Method (CPRS, 2, Serialized)
                    {
                        Local1 = ResourceTemplate ()
                            {
                                StartDependentFn (0x00, 0x01)
                                {
                                    IO (Decode16,
                                        0x0000,             // Range Minimum
                                        0x0000,             // Range Maximum
                                        0x08,               // Alignment
                                        0x08,               // Length
                                        )
                                    IRQNoFlags ()
                                        {}
                                }
                                StartDependentFnNoPri ()
                                {
                                    IO (Decode16,
                                        0x03F8,             // Range Minimum
                                        0x03F8,             // Range Maximum
                                        0x08,               // Alignment
                                        0x08,               // Length
                                        )
                                    IRQNoFlags ()
                                        {3,4,5,6,7,9,10,11,12}
                                }
                                StartDependentFnNoPri ()
                                {
                                    IO (Decode16,
                                        0x02F8,             // Range Minimum
                                        0x02F8,             // Range Maximum
                                        0x08,               // Alignment
                                        0x08,               // Length
                                        )
                                    IRQNoFlags ()
                                        {3,4,5,6,7,9,10,11,12}
                                }
                                StartDependentFnNoPri ()
                                {
                                    IO (Decode16,
                                        0x03E8,             // Range Minimum
                                        0x03E8,             // Range Maximum
                                        0x08,               // Alignment
                                        0x08,               // Length
                                        )
                                    IRQNoFlags ()
                                        {3,4,5,6,7,9,10,11,12}
                                }
                                StartDependentFnNoPri ()
                                {
                                    IO (Decode16,
                                        0x02E8,             // Range Minimum
                                        0x02E8,             // Range Maximum
                                        0x08,               // Alignment
                                        0x08,               // Length
                                        )
                                    IRQNoFlags ()
                                        {3,4,5,6,7,9,10,11,12}
                                }
                                StartDependentFn (0x02, 0x01)
                                {
                                    IO (Decode16,
                                        0x0100,             // Range Minimum
                                        0x03F8,             // Range Maximum
                                        0x08,               // Alignment
                                        0x08,               // Length
                                        )
                                    IRQNoFlags ()
                                        {3,4,5,6,7,9,10,11,12}
                                }
                                EndDependentFn ()
                            }
                        CreateWordField (Local1, 0x04, IOAL)
                        CreateWordField (Local1, 0x06, IOAH)
                        CreateWordField (Local1, 0x0B, IRQW)
                        IOAL = Arg0
                        IOAH = Arg0
                        IRQW = (One << Arg1)
                        Return (Local1)
                    }

                    Device (LPT)
                    {
                        Name (_HID, EisaId ("PNP0400") /* Standard LPT Parallel Port */)  // _HID: Hardware ID
                        Name (_UID, One)  // _UID: Unique ID
                        Name (_DDN, "LPT1")  // _DDN: DOS Device Name
                        Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
                        {
                            Return (SCRS (PBAL, PBAH, PPIR, PEPP))
                        }

                        Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                        {
                            CreateWordField (Arg0, 0x02, IOAL)
                            CreateWordField (Arg0, 0x09, IRQW)
                            FLPT = Zero
                            PBAH = ((IOAL & 0xFC00) >> 0x08)
                            PBAL = ((IOAL & 0x03FC) >> 0x02)
                            If (IRQW)
                            {
                                FindSetRightBit (IRQW, Local0)
                                PPIR = (Local0 - One)
                            }
                            Else
                            {
                                PPIR = Zero
                            }

                            FLPT = One
                        }

                        Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                        {
                            Return (LPRS (PEPP, 0x0378, 0x05, 0x1EF8))
                        }

                        Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                        {
                            FLPT = Zero
                        }

                        Method (_STA, 0, NotSerialized)  // _STA: Status
                        {
                            If ((SIOC & One))
                            {
                                If (FLPT)
                                {
                                    Return (0x0F)
                                }

                                If (PBAL)
                                {
                                    Return (0x0D)
                                }

                                PBAL = Ones
                                If (PBAL)
                                {
                                    Return (0x0D)
                                }
                            }

                            Return (Zero)
                        }
                    }

                    Device (COMA)
                    {
                        Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                        Name (_UID, One)  // _UID: Unique ID
                        Name (_DDN, "COM1")  // _DDN: DOS Device Name
                        Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                        {
                            Return (SCRS (S1BL, S1BH, CAIR, One))
                        }

                        Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                        {
                            CreateWordField (Arg0, 0x02, IOAL)
                            CreateWordField (Arg0, 0x09, IRQW)
                            FCMA = Zero
                            S1BH = ((IOAL & 0xFC00) >> 0x08)
                            S1BL = ((IOAL & 0x03F8) >> 0x02)
                            If (IRQW)
                            {
                                FindSetRightBit (IRQW, Local0)
                                CAIR = (Local0 - One)
                            }
                            Else
                            {
                                CAIR = Zero
                            }

                            FCMA = One
                        }

                        Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                        {
                            Return (CPRS (0x03F8, 0x04))
                        }

                        Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                        {
                            FCMA = Zero
                        }

                        Method (_STA, 0, NotSerialized)  // _STA: Status
                        {
                            If ((SIOC & 0x02))
                            {
                                If (FCMA)
                                {
                                    Return (0x0F)
                                }

                                If (S1BL)
                                {
                                    Return (0x0D)
                                }

                                S1BL = Ones
                                If (S1BL)
                                {
                                    Return (0x0D)
                                }
                            }

                            Return (Zero)
                        }
                    }

                    Device (COMB)
                    {
                        Name (_HID, EisaId ("PNP0501") /* 16550A-compatible COM Serial Port */)  // _HID: Hardware ID
                        Name (_UID, 0x02)  // _UID: Unique ID
                        Name (_DDN, "COM2")  // _DDN: DOS Device Name
                        Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                        {
                            Return (SCRS (S2BL, S2BH, CBIR, One))
                        }

                        Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                        {
                            CreateWordField (Arg0, 0x02, IOAL)
                            CreateWordField (Arg0, 0x09, IRQW)
                            FCMB = Zero
                            S2BH = ((IOAL & 0xFC00) >> 0x08)
                            S2BL = ((IOAL & 0x03F8) >> 0x02)
                            If (IRQW)
                            {
                                FindSetRightBit (IRQW, Local0)
                                CBIR = (Local0 - One)
                            }
                            Else
                            {
                                CBIR = Zero
                            }

                            FCMB = One
                        }

                        Method (_PRS, 0, NotSerialized)  // _PRS: Possible Resource Settings
                        {
                            Return (CPRS (0x02F8, 0x03))
                        }

                        Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                        {
                            FCMB = Zero
                        }

                        Method (_STA, 0, NotSerialized)  // _STA: Status
                        {
                            If ((SIOC & 0x04))
                            {
                                If (FCMB)
                                {
                                    Return (0x0F)
                                }

                                If (S2BL)
                                {
                                    Return (0x0D)
                                }

                                S2BL = Ones
                                If (S2BL)
                                {
                                    Return (0x0D)
                                }
                            }

                            Return (Zero)
                        }
                    }

                    Device (FDC)
                    {
                        Name (_HID, EisaId ("PNP0700"))  // _HID: Hardware ID
                        Name (RSRC, ResourceTemplate ()
                        {
                            IO (Decode16,
                                0x0000,             // Range Minimum
                                0x0000,             // Range Maximum
                                0x01,               // Alignment
                                0x06,               // Length
                                )
                            IO (Decode16,
                                0x0000,             // Range Minimum
                                0x0000,             // Range Maximum
                                0x01,               // Alignment
                                0x01,               // Length
                                )
                            IRQNoFlags ()
                                {}
                            DMA (Compatibility, NotBusMaster, Transfer8, )
                                {}
                        })
                        Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                        {
                            Local1 = RSRC /* \_SB_.PCI0.ISA_.SIO_.FDC_.RSRC */
                            CreateWordField (Local1, 0x02, IOAL)
                            CreateWordField (Local1, 0x04, IOAH)
                            CreateWordField (Local1, 0x0A, IOBL)
                            CreateWordField (Local1, 0x0C, IOBH)
                            CreateWordField (Local1, 0x11, IRQW)
                            CreateByteField (Local1, 0x14, DMAB)
                            Local0 = (((FBAH & 0xFC) << 0x08) | ((FBAL & 
                                0xFE) << 0x02))
                            IOAL = Local0
                            IOAH = Local0
                            Local0 += 0x07
                            IOBL = Local0
                            IOBH = Local0
                            Local0 = FCIR /* \_SB_.PCI0.ISA_.SIO_.FCIR */
                            If (Local0)
                            {
                                IRQW = (One << Local0)
                            }

                            Local0 = CDMA /* \_SB_.PCI0.ISA_.SIO_.CDMA */
                            DMAB = ((One << Local0) >> One)
                            Return (Local1)
                        }

                        Method (_SRS, 1, Serialized)  // _SRS: Set Resource Settings
                        {
                            CreateWordField (Arg0, 0x02, IOAL)
                            CreateWordField (Arg0, 0x11, IRQW)
                            CreateByteField (Arg0, 0x14, DMAB)
                            FDCA = Zero
                            FBAH = ((IOAL & 0xFC00) >> 0x08)
                            FBAL = ((IOAL & 0x03F8) >> 0x02)
                            If (IRQW)
                            {
                                FindSetRightBit (IRQW, Local0)
                                FCIR = (Local0 - One)
                            }
                            Else
                            {
                                FCIR = Zero
                            }

                            If (DMAB)
                            {
                                FindSetRightBit (DMAB, CDMA) /* \_SB_.PCI0.ISA_.SIO_.CDMA */
                            }
                            Else
                            {
                                CDMA = Zero
                            }

                            FDCA = One
                        }

                        Name (_PRS, ResourceTemplate ()  // _PRS: Possible Resource Settings
                        {
                            StartDependentFn (0x00, 0x01)
                            {
                                IO (Decode16,
                                    0x03F0,             // Range Minimum
                                    0x03F0,             // Range Maximum
                                    0x01,               // Alignment
                                    0x06,               // Length
                                    )
                                IO (Decode16,
                                    0x03F7,             // Range Minimum
                                    0x03F7,             // Range Maximum
                                    0x01,               // Alignment
                                    0x01,               // Length
                                    )
                                IRQNoFlags ()
                                    {6}
                                DMA (Compatibility, NotBusMaster, Transfer8, )
                                    {2}
                            }
                            StartDependentFnNoPri ()
                            {
                                IO (Decode16,
                                    0x03F0,             // Range Minimum
                                    0x03F0,             // Range Maximum
                                    0x01,               // Alignment
                                    0x06,               // Length
                                    )
                                IO (Decode16,
                                    0x03F7,             // Range Minimum
                                    0x03F7,             // Range Maximum
                                    0x01,               // Alignment
                                    0x01,               // Length
                                    )
                                IRQNoFlags ()
                                    {3,4,5,6,7,9,10,11,12}
                                DMA (Compatibility, NotBusMaster, Transfer8, )
                                    {0,1,2,3}
                            }
                            StartDependentFnNoPri ()
                            {
                                IO (Decode16,
                                    0x0370,             // Range Minimum
                                    0x0370,             // Range Maximum
                                    0x01,               // Alignment
                                    0x06,               // Length
                                    )
                                IO (Decode16,
                                    0x0377,             // Range Minimum
                                    0x0377,             // Range Maximum
                                    0x01,               // Alignment
                                    0x01,               // Length
                                    )
                                IRQNoFlags ()
                                    {3,4,5,6,7,9,10,11,12}
                                DMA (Compatibility, NotBusMaster, Transfer8, )
                                    {0,1,2,3}
                            }
                            EndDependentFn ()
                        })
                        Method (_DIS, 0, NotSerialized)  // _DIS: Disable Device
                        {
                            FDCA = Zero
                        }

                        Method (_STA, 0, NotSerialized)  // _STA: Status
                        {
                            If ((SIOC & 0x08))
                            {
                                If (FDCA)
                                {
                                    Return (0x0F)
                                }

                                If (FBAL)
                                {
                                    Return (0x0D)
                                }

                                FBAL = Ones
                                If (FBAL)
                                {
                                    Return (0x0D)
                                }
                            }

                            Return (Zero)
                        }
                    }

                    Device (LDEV)
                    {
                        Name (_HID, EisaId ("PNP0C02") /* PNP Motherboard Resources */)  // _HID: Hardware ID
                        Name (_UID, 0x05)  // _UID: Unique ID
                        Method (_STA, 0, NotSerialized)  // _STA: Status
                        {
                            If (LDDV)
                            {
                                Return (0x0F)
                            }

                            Return (Zero)
                        }

                        Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                        {
                            Local0 = Buffer (0x02)
                                {
                                     0x79, 0x00                                       // y.
                                }
                            If ((LDDV & 0x80))
                            {
                                Concatenate (Buffer (0x10)
                                    {
                                        /* 0000 */  0x47, 0x01, 0xF0, 0x03, 0xF0, 0x03, 0x01, 0x06,  // G.......
                                        /* 0008 */  0x47, 0x01, 0xF7, 0x03, 0xF7, 0x03, 0x01, 0x01   // G.......
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            If ((LDDV & One))
                            {
                                Concatenate (Buffer (0x08)
                                    {
                                         0x47, 0x01, 0xF8, 0x03, 0xF8, 0x03, 0x01, 0x08   // G.......
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            If ((LDDV & 0x02))
                            {
                                Concatenate (Buffer (0x08)
                                    {
                                         0x47, 0x01, 0xF8, 0x02, 0xF8, 0x02, 0x01, 0x08   // G.......
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            If ((LDDV & 0x04))
                            {
                                Concatenate (Buffer (0x08)
                                    {
                                         0x47, 0x01, 0xE8, 0x03, 0xE8, 0x03, 0x01, 0x08   // G.......
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            If ((LDDV & 0x08))
                            {
                                Concatenate (Buffer (0x08)
                                    {
                                         0x47, 0x01, 0xE8, 0x02, 0xE8, 0x02, 0x01, 0x08   // G.......
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            If ((LDDV & 0x10))
                            {
                                Concatenate (Buffer (0x08)
                                    {
                                         0x47, 0x01, 0x78, 0x03, 0x78, 0x03, 0x01, 0x08   // G.x.x...
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            If ((LDDV & 0x20))
                            {
                                Concatenate (Buffer (0x08)
                                    {
                                         0x47, 0x01, 0x78, 0x02, 0x78, 0x02, 0x01, 0x08   // G.x.x...
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            If ((LDDV & 0x40))
                            {
                                Concatenate (Buffer (0x08)
                                    {
                                         0x47, 0x01, 0xBC, 0x03, 0xBC, 0x03, 0x01, 0x04   // G.......
                                    }, Local0, Local1)
                                Local0 = Local1
                            }

                            Return (Local0)
                        }
                    }
                }
            }

            Device (PWR)
            {
                Name (_ADR, 0x00070003)  // _ADR: Address
                OperationRegion (PCI, PCI_Config, 0x40, 0x60)
                Field (PCI, DWordAcc, NoLock, Preserve)
                {
                    PMBA,   32, 
                    Offset (0x50), 
                    SBBA,   32
                }
            }

            Device (USB)
            {
                Name (_ADR, 0x00070002)  // _ADR: Address
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x08, 
                    One
                })
            }

            Device (IDE)
            {
                Name (_ADR, 0x00070001)  // _ADR: Address
            }

            OperationRegion (RE01, PCI_Config, 0x40, 0x04)
            Field (RE01, DWordAcc, NoLock, Preserve)
            {
                ECIO,   32
            }

            OperationRegion (RE02, PCI_Config, 0xC4, 0x04)
            Field (RE02, DWordAcc, NoLock, Preserve)
            {
                VMME,   1, 
                VMMS,   3, 
                    ,   16, 
                VMMB,   12
            }

            Name (SUPP, Zero)
            Name (CTRL, Zero)
            Method (_OSC, 4, Serialized)  // _OSC: Operating System Capabilities
            {
                If ((Arg0 == ToUUID ("33db4d5b-1ff7-401c-9657-7441c03dd766") /* PCI Host Bridge Device */))
                {
                    CreateDWordField (Arg3, Zero, CDW1)
                    CreateDWordField (Arg3, 0x04, CDW2)
                    CreateDWordField (Arg3, 0x08, CDW3)
                    SUPP = CDW2 /* \_SB_.PCI0._OSC.CDW2 */
                    CTRL = CDW3 /* \_SB_.PCI0._OSC.CDW3 */
                    Name (HPIN, 0x3F)
                    CTRL &= POSC /* \_SB_.POSC */
                    If (!(CDW1 & One))
                    {
                        If ((CTRL & One))
                        {
                            HPIN &= 0xFFFFFFFD /* \_SB_.PCI0._OSC.HPIN */
                        }

                        If ((CTRL & 0x02))
                        {
                            HPIN &= 0xFFFFFFFE /* \_SB_.PCI0._OSC.HPIN */
                        }

                        If ((CTRL & 0x04)) {}
                        If ((CTRL & 0x10)) {}
                    }

                    Local0 = DCFI /* \_SB_.DCFI */
                    DCFI = ((Local0 & 0xFFFFFFC0) | HPIN)
                    If ((Arg1 != One))
                    {
                        CDW1 |= 0x08
                    }

                    If ((CDW3 != CTRL))
                    {
                        CDW1 |= 0x10
                    }

                    CDW3 = CTRL /* \_SB_.PCI0.CTRL */
                    Return (Arg3)
                }
                Else
                {
                    CDW1 |= 0x04
                    Return (Arg3)
                }
            }

            Device (EXPL)
            {
                Name (_HID, EisaId ("PNP0C02") /* PNP Motherboard Resources */)  // _HID: Hardware ID
                Name (_UID, 0x04)  // _UID: Unique ID
                Method (_STA, 0, NotSerialized)  // _STA: Status
                {
                    If (CCAP)
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }

                Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                {
                    Name (BUF1, ResourceTemplate ()
                    {
                        DWordMemory (ResourceConsumer, PosDecode, MinFixed, MaxFixed, NonCacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x00000000,         // Range Minimum
                            0x00000000,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00000000,         // Length
                            ,, _Y03, AddressRangeMemory, TypeStatic)
                        IO (Decode16,
                            0x0000,             // Range Minimum
                            0x0000,             // Range Maximum
                            0x01,               // Alignment
                            0x00,               // Length
                            _Y04)
                        DWordMemory (ResourceConsumer, PosDecode, MinFixed, MaxFixed, NonCacheable, ReadWrite,
                            0x00000000,         // Granularity
                            0x00000000,         // Range Minimum
                            0x00000000,         // Range Maximum
                            0x00000000,         // Translation Offset
                            0x00000000,         // Length
                            ,, _Y05, AddressRangeMemory, TypeStatic)
                    })
                    If (VMME)
                    {
                        CreateDWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y03._MIN, MCMN)  // _MIN: Minimum Base Address
                        CreateDWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y03._MAX, MCMX)  // _MAX: Maximum Base Address
                        CreateDWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y03._LEN, MCLN)  // _LEN: Length
                        MCMN = (VMMB << 0x14)
                        MCLN = (0x10000000 >> VMMS)
                        MCMX = ((MCMN + MCLN) - One)
                    }

                    If (ECIO)
                    {
                        CreateWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y04._MIN, IOMN)  // _MIN: Minimum Base Address
                        CreateWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y04._MAX, IOMX)  // _MAX: Maximum Base Address
                        CreateByteField (BUF1, \_SB.PCI0.EXPL._CRS._Y04._LEN, IOLN)  // _LEN: Length
                        IOMN = (ECIO & 0xFFFFFFFE)
                        IOMX = IOMN /* \_SB_.PCI0.EXPL._CRS.IOMN */
                        IOLN = 0x20
                    }

                    If (ECFG)
                    {
                        CreateDWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y05._MIN, MIMN)  // _MIN: Minimum Base Address
                        CreateDWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y05._MAX, MIMX)  // _MAX: Maximum Base Address
                        CreateDWordField (BUF1, \_SB.PCI0.EXPL._CRS._Y05._LEN, MILN)  // _LEN: Length
                        MIMN = ECFG /* \_SB_.ECFG */
                        MILN = 0x00200000
                        MIMX = ((MIMN + MILN) - One)
                    }

                    Return (BUF1) /* \_SB_.PCI0.EXPL._CRS.BUF1 */
                }
            }

            Device (DMAR)
            {
                Name (_HID, EisaId ("PNP0C02") /* PNP Motherboard Resources */)  // _HID: Hardware ID
                Name (_UID, 0x06)  // _UID: Unique ID
                Method (_STA, 0, NotSerialized)  // _STA: Status
                {
                    Local1 = (VVTS | AIOS) /* \_SB_.AIOS */
                    If (Local1)
                    {
                        Return (0x0F)
                    }

                    Return (Zero)
                }

                Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
                {
                    Name (TMPL, ResourceTemplate ()
                    {
                        Memory32Fixed (ReadWrite,
                            0x00000000,         // Address Base
                            0x00000000,         // Address Length
                            _Y06)
                    })
                    CreateDWordField (TMPL, \_SB.PCI0.DMAR._CRS._Y06._BAS, BASE)  // _BAS: Base Address
                    CreateDWordField (TMPL, \_SB.PCI0.DMAR._CRS._Y06._LEN, SIZE)  // _LEN: Length
                    Local1 = (VVTB | AIOB) /* \_SB_.AIOB */
                    Local2 = (VVTS | AIOS) /* \_SB_.AIOS */
                    If (Local2)
                    {
                        BASE = Local1
                        SIZE = Local2
                    }

                    Return (TMPL) /* \_SB_.PCI0.DMAR._CRS.TMPL */
                }
            }

            Device (VMGC)
            {
                Name (_HID, "VMW0001")  // _HID: Hardware ID
                Name (_CID, Package (0x02)  // _CID: Compatible ID
                {
                    "VM_Gen_Counter", 
                    EisaId ("PNP0C02") /* PNP Motherboard Resources */
                })
                Name (_UID, 0x07)  // _UID: Unique ID
                Name (_DDN, "VM_Gen_Counter")  // _DDN: DOS Device Name
                Method (_INI, 0, Serialized)  // _INI: Initialize
                {
                    If ((TOOS == 0x1000))
                    {
                        _CID = 0x020CD041
                    }
                }

                Method (_STA, 0, NotSerialized)  // _STA: Status
                {
                    Local0 = GENL /* \_SB_.GENL */
                    If ((Local0 & One))
                    {
                        Return (0x0B)
                    }

                    Return (Zero)
                }

                Method (ADDR, 0, Serialized)
                {
                    Local0 = Package (0x02) {}
                    Local0 [Zero] = (GENL & 0xFFFFFFF0)
                    Local0 [One] = GENH /* \_SB_.GENH */
                    Return (Local0)
                }
            }

            Method (DVHP, 0, Serialized)
            {
                Local0 = HPCM /* \_SB_.HPCM */
                Local5 = HPSL /* \_SB_.HPSL */
                Local2 = (0xFFFF & Local5)
                Local1 = (Local5 >> 0x10)
                If ((Local1 < 0x10))
                {
                    Local1 = ((Local1 << 0x03) + 0x88)
                }

                Local3 = (Local1 & 0x07)
                Local4 = (Local1 >> 0x03)
                If ((Local0 == One))
                {
                    Local6 = Zero
                    Local2 = 0xFFFF
                }
                ElseIf ((Local0 == 0x02))
                {
                    Local6 = 0x03
                }
                Else
                {
                    HPST = Ones
                    Return (Zero)
                }

                HPST = ^^PPHR (Local1, Local2, Local6)
            }

            Method (PSHP, 1, NotSerialized)
            {
                Local2 = ((Arg0 - 0x00110000) >> 0x10)
                Local1 = (0x04 << Local2)
                Local1 |= One
                Local0 = DCFI /* \_SB_.DCFI */
                DCFI = (Local0 & ~Local1)
            }

            Device (P2P0)
            {
                Name (_ADR, 0x00110000)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (OSHP, 0, NotSerialized)
                {
                    PSHP (_ADR)
                }

                Method (PPHR, 2, NotSerialized)
                {
                    Local0 = (Arg0 & One)
                    Local1 = (Arg0 & 0x02)
                    Local2 = (Arg0 & 0x04)
                    Local3 = (Arg0 & 0x08)
                    While (One)
                    {
                        If ((0x0F < Arg0))
                        {
                            If (Local3)
                            {
                                If (Local2)
                                {
                                    If (Local1)
                                    {
                                        If (Local0)
                                        {
                                            Notify (S32F, Arg1)
                                            Break
                                        }

                                        Notify (S31F, Arg1)
                                        Break
                                    }

                                    If (Local0)
                                    {
                                        Notify (S30F, Arg1)
                                        Break
                                    }

                                    Notify (S29F, Arg1)
                                    Break
                                }

                                If (Local1)
                                {
                                    If (Local0)
                                    {
                                        Notify (S28F, Arg1)
                                        Break
                                    }

                                    Notify (S27F, Arg1)
                                    Break
                                }

                                If (Local0)
                                {
                                    Notify (S26F, Arg1)
                                    Break
                                }

                                Notify (S25F, Arg1)
                                Break
                            }

                            If (Local2)
                            {
                                If (Local1)
                                {
                                    If (Local0)
                                    {
                                        Notify (S24F, Arg1)
                                        Break
                                    }

                                    Notify (S23F, Arg1)
                                    Break
                                }

                                If (Local0)
                                {
                                    Notify (S22F, Arg1)
                                    Break
                                }

                                Notify (S21F, Arg1)
                                Break
                            }

                            If (Local1)
                            {
                                If (Local0)
                                {
                                    Notify (S20F, Arg1)
                                    Break
                                }

                                Notify (S19F, Arg1)
                                Break
                            }

                            If (Local0)
                            {
                                Notify (S18F, Arg1)
                                Break
                            }

                            Notify (S17F, Arg1)
                            Break
                        }

                        If (Local3)
                        {
                            If (Local2)
                            {
                                If (Local1)
                                {
                                    If (Local0)
                                    {
                                        Notify (S16F, Arg1)
                                        Break
                                    }

                                    Notify (S15F, Arg1)
                                    Break
                                }

                                If (Local0)
                                {
                                    Notify (S14F, Arg1)
                                    Break
                                }

                                Notify (S13F, Arg1)
                                Break
                            }

                            If (Local1)
                            {
                                If (Local0)
                                {
                                    Notify (S12F, Arg1)
                                    Break
                                }

                                Notify (S11F, Arg1)
                                Break
                            }

                            If (Local0)
                            {
                                Notify (S10F, Arg1)
                                Break
                            }

                            Notify (S9F0, Arg1)
                            Break
                        }

                        If (Local2)
                        {
                            If (Local1)
                            {
                                If (Local0)
                                {
                                    Notify (S8F0, Arg1)
                                    Break
                                }

                                Notify (S7F0, Arg1)
                                Break
                            }

                            If (Local0)
                            {
                                Notify (S6F0, Arg1)
                                Break
                            }

                            Notify (S5F0, Arg1)
                            Break
                        }

                        If (Local1)
                        {
                            If (Local0)
                            {
                                Notify (S4F0, Arg1)
                                Break
                            }

                            Notify (S3F0, Arg1)
                            Break
                        }

                        If (Local0)
                        {
                            Notify (S2F0, Arg1)
                            Break
                        }

                        Notify (S1F0, Arg1)
                        Break
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0020)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S2F0)
                {
                    Name (_ADR, 0x00010000)  // _ADR: Address
                    Name (_SUN, 0x0021)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S3F0)
                {
                    Name (_ADR, 0x00020000)  // _ADR: Address
                    Name (_SUN, 0x0022)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S4F0)
                {
                    Name (_ADR, 0x00030000)  // _ADR: Address
                    Name (_SUN, 0x0023)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S5F0)
                {
                    Name (_ADR, 0x00040000)  // _ADR: Address
                    Name (_SUN, 0x0024)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S6F0)
                {
                    Name (_ADR, 0x00050000)  // _ADR: Address
                    Name (_SUN, 0x0025)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S7F0)
                {
                    Name (_ADR, 0x00060000)  // _ADR: Address
                    Name (_SUN, 0x0026)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S8F0)
                {
                    Name (_ADR, 0x00070000)  // _ADR: Address
                    Name (_SUN, 0x0027)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S9F0)
                {
                    Name (_ADR, 0x00080000)  // _ADR: Address
                    Name (_SUN, 0x0028)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S10F)
                {
                    Name (_ADR, 0x00090000)  // _ADR: Address
                    Name (_SUN, 0x0029)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S11F)
                {
                    Name (_ADR, 0x000A0000)  // _ADR: Address
                    Name (_SUN, 0x002A)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S12F)
                {
                    Name (_ADR, 0x000B0000)  // _ADR: Address
                    Name (_SUN, 0x002B)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S13F)
                {
                    Name (_ADR, 0x000C0000)  // _ADR: Address
                    Name (_SUN, 0x002C)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S14F)
                {
                    Name (_ADR, 0x000D0000)  // _ADR: Address
                    Name (_SUN, 0x002D)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S15F)
                {
                    Name (_ADR, 0x000E0000)  // _ADR: Address
                    Name (_SUN, 0x002E)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S16F)
                {
                    Name (_ADR, 0x000F0000)  // _ADR: Address
                    Name (_SUN, 0x002F)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S17F)
                {
                    Name (_ADR, 0x00100000)  // _ADR: Address
                    Name (_SUN, 0x0030)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S18F)
                {
                    Name (_ADR, 0x00110000)  // _ADR: Address
                    Name (_SUN, 0x0031)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S19F)
                {
                    Name (_ADR, 0x00120000)  // _ADR: Address
                    Name (_SUN, 0x0032)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S20F)
                {
                    Name (_ADR, 0x00130000)  // _ADR: Address
                    Name (_SUN, 0x0033)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S21F)
                {
                    Name (_ADR, 0x00140000)  // _ADR: Address
                    Name (_SUN, 0x0034)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S22F)
                {
                    Name (_ADR, 0x00150000)  // _ADR: Address
                    Name (_SUN, 0x0035)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S23F)
                {
                    Name (_ADR, 0x00160000)  // _ADR: Address
                    Name (_SUN, 0x0036)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S24F)
                {
                    Name (_ADR, 0x00170000)  // _ADR: Address
                    Name (_SUN, 0x0037)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S25F)
                {
                    Name (_ADR, 0x00180000)  // _ADR: Address
                    Name (_SUN, 0x0038)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S26F)
                {
                    Name (_ADR, 0x00190000)  // _ADR: Address
                    Name (_SUN, 0x0039)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S27F)
                {
                    Name (_ADR, 0x001A0000)  // _ADR: Address
                    Name (_SUN, 0x003A)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S28F)
                {
                    Name (_ADR, 0x001B0000)  // _ADR: Address
                    Name (_SUN, 0x003B)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S29F)
                {
                    Name (_ADR, 0x001C0000)  // _ADR: Address
                    Name (_SUN, 0x003C)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S30F)
                {
                    Name (_ADR, 0x001D0000)  // _ADR: Address
                    Name (_SUN, 0x003D)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S31F)
                {
                    Name (_ADR, 0x001E0000)  // _ADR: Address
                    Name (_SUN, 0x003E)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }

                Device (S32F)
                {
                    Name (_ADR, 0x001F0000)  // _ADR: Address
                    Name (_SUN, 0x003F)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, _ADR)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, _ADR))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE40)
            {
                Name (_ADR, 0x00150000)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A0)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE41)
            {
                Name (_ADR, 0x00150001)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A1)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE42)
            {
                Name (_ADR, 0x00150002)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A2)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE43)
            {
                Name (_ADR, 0x00150003)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A3)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE44)
            {
                Name (_ADR, 0x00150004)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A4)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE45)
            {
                Name (_ADR, 0x00150005)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A5)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE46)
            {
                Name (_ADR, 0x00150006)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A6)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE47)
            {
                Name (_ADR, 0x00150007)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00A7)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE50)
            {
                Name (_ADR, 0x00160000)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C0)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE51)
            {
                Name (_ADR, 0x00160001)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C1)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE52)
            {
                Name (_ADR, 0x00160002)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C2)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE53)
            {
                Name (_ADR, 0x00160003)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C3)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE54)
            {
                Name (_ADR, 0x00160004)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C4)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE55)
            {
                Name (_ADR, 0x00160005)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C5)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE56)
            {
                Name (_ADR, 0x00160006)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C6)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE57)
            {
                Name (_ADR, 0x00160007)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00C7)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE60)
            {
                Name (_ADR, 0x00170000)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E0)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE61)
            {
                Name (_ADR, 0x00170001)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E1)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE62)
            {
                Name (_ADR, 0x00170002)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E2)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE63)
            {
                Name (_ADR, 0x00170003)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E3)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE64)
            {
                Name (_ADR, 0x00170004)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E4)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE65)
            {
                Name (_ADR, 0x00170005)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E5)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE66)
            {
                Name (_ADR, 0x00170006)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E6)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE67)
            {
                Name (_ADR, 0x00170007)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x00E7)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE70)
            {
                Name (_ADR, 0x00180000)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0100)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE71)
            {
                Name (_ADR, 0x00180001)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0101)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE72)
            {
                Name (_ADR, 0x00180002)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0102)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE73)
            {
                Name (_ADR, 0x00180003)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0103)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE74)
            {
                Name (_ADR, 0x00180004)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0104)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE75)
            {
                Name (_ADR, 0x00180005)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0105)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE76)
            {
                Name (_ADR, 0x00180006)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0106)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Device (PE77)
            {
                Name (_ADR, 0x00180007)  // _ADR: Address
                Name (_HPP, Package (0x04)  // _HPP: Hot Plug Parameters
                {
                    0x08, 
                    0x40, 
                    One, 
                    Zero
                })
                Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                {
                    0x03, 
                    0x03
                })
                Method (BEJ0, 2, NotSerialized)
                {
                    PEJ0 (Arg0, Arg1, _ADR)
                }

                Method (BDSM, 5, Serialized)
                {
                    Return (PDSM (Arg0, Arg1, Arg2, Arg3, Arg4, _ADR))
                }

                Method (PPHR, 2, NotSerialized)
                {
                    If (!Arg0)
                    {
                        Notify (S1F0, Arg1)
                    }

                    Return (Zero)
                }

                Device (S1F0)
                {
                    Name (_ADR, Zero)  // _ADR: Address
                    Name (_SUN, 0x0107)  // _SUN: Slot User Number
                    OperationRegion (REGS, PCI_Config, Zero, 0x02)
                    Field (REGS, WordAcc, NoLock, Preserve)
                    {
                        ID,     16
                    }

                    Method (_STA, 0, NotSerialized)  // _STA: Status
                    {
                        Return (PSTA (ID))
                    }

                    Method (_EJ0, 1, NotSerialized)  // _EJx: Eject Device, x=0-9
                    {
                        BEJ0 (Arg0, Zero)
                    }

                    Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                    {
                        Return (BDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                    }

                    Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
                    {
                        0x03, 
                        0x03
                    })
                }
            }

            Name (_BBN, 0x00)  // _BBN: BIOS Bus Number
            Name (XCRS, Buffer (0xCC)
            {
                /* 0000 */  0x88, 0x0D, 0x00, 0x02, 0x0C, 0x00, 0x00, 0x00,  // ........
                /* 0008 */  0x00, 0x00, 0x7F, 0x00, 0x00, 0x00, 0x80, 0x00,  // ........
                /* 0010 */  0x88, 0x0D, 0x00, 0x01, 0x0C, 0x03, 0x00, 0x00,  // ........
                /* 0018 */  0x00, 0x0D, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xF3,  // ........
                /* 0020 */  0x88, 0x0D, 0x00, 0x01, 0x0C, 0x03, 0x00, 0x00,  // ........
                /* 0028 */  0x00, 0x00, 0xF7, 0x0C, 0x00, 0x00, 0xF8, 0x0C,  // ........
                /* 0030 */  0x87, 0x17, 0x00, 0x00, 0x0C, 0x01, 0x00, 0x00,  // ........
                /* 0038 */  0x00, 0x00, 0x00, 0x00, 0xF0, 0xFE, 0xFF, 0xFF,  // ........
                /* 0040 */  0xDF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                /* 0048 */  0xF0, 0x00, 0x87, 0x17, 0x00, 0x00, 0x0C, 0x01,  // ........
                /* 0050 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0xD4, 0xFE,  // .....P..
                /* 0058 */  0xFF, 0xFF, 0xDF, 0xFE, 0x00, 0x00, 0x00, 0x00,  // ........
                /* 0060 */  0x00, 0xB0, 0x0B, 0x00, 0x87, 0x17, 0x00, 0x00,  // ........
                /* 0068 */  0x0C, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                /* 0070 */  0xC1, 0xFE, 0xFF, 0xFF, 0xD3, 0xFE, 0x00, 0x00,  // ........
                /* 0078 */  0x00, 0x00, 0x00, 0x00, 0x13, 0x00, 0x87, 0x17,  // ........
                /* 0080 */  0x00, 0x00, 0x0C, 0x01, 0x00, 0x00, 0x00, 0x00,  // ........
                /* 0088 */  0x00, 0x00, 0x50, 0xFA, 0xFF, 0xFF, 0xBF, 0xFE,  // ..P.....
                /* 0090 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x04,  // ......p.
                /* 0098 */  0x87, 0x17, 0x00, 0x00, 0x0C, 0x07, 0x00, 0x00,  // ........
                /* 00A0 */  0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0xFF, 0xFF,  // ........
                /* 00A8 */  0x4F, 0xFA, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // O.......
                /* 00B0 */  0x50, 0x0A, 0x87, 0x17, 0x00, 0x00, 0x0C, 0x01,  // P.......
                /* 00B8 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0,  // ........
                /* 00C0 */  0xFF, 0xFF, 0xFF, 0xEF, 0x00, 0x00, 0x00, 0x00,  // ........
                /* 00C8 */  0x00, 0x00, 0x00, 0x30                           // ...0
            })
            Method (PPHR, 3, NotSerialized)
            {
                If ((Arg0 == 0xC7))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE77, Arg2)
                        Return (Zero)
                    }

                    Return (^PE77.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xC6))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE76, Arg2)
                        Return (Zero)
                    }

                    Return (^PE76.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xC5))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE75, Arg2)
                        Return (Zero)
                    }

                    Return (^PE75.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xC4))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE74, Arg2)
                        Return (Zero)
                    }

                    Return (^PE74.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xC3))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE73, Arg2)
                        Return (Zero)
                    }

                    Return (^PE73.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xC2))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE72, Arg2)
                        Return (Zero)
                    }

                    Return (^PE72.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xC1))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE71, Arg2)
                        Return (Zero)
                    }

                    Return (^PE71.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xC0))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE70, Arg2)
                        Return (Zero)
                    }

                    Return (^PE70.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xBF))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE67, Arg2)
                        Return (Zero)
                    }

                    Return (^PE67.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xBE))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE66, Arg2)
                        Return (Zero)
                    }

                    Return (^PE66.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xBD))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE65, Arg2)
                        Return (Zero)
                    }

                    Return (^PE65.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xBC))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE64, Arg2)
                        Return (Zero)
                    }

                    Return (^PE64.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xBB))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE63, Arg2)
                        Return (Zero)
                    }

                    Return (^PE63.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xBA))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE62, Arg2)
                        Return (Zero)
                    }

                    Return (^PE62.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB9))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE61, Arg2)
                        Return (Zero)
                    }

                    Return (^PE61.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB8))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE60, Arg2)
                        Return (Zero)
                    }

                    Return (^PE60.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB7))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE57, Arg2)
                        Return (Zero)
                    }

                    Return (^PE57.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB6))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE56, Arg2)
                        Return (Zero)
                    }

                    Return (^PE56.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB5))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE55, Arg2)
                        Return (Zero)
                    }

                    Return (^PE55.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB4))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE54, Arg2)
                        Return (Zero)
                    }

                    Return (^PE54.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB3))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE53, Arg2)
                        Return (Zero)
                    }

                    Return (^PE53.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB2))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE52, Arg2)
                        Return (Zero)
                    }

                    Return (^PE52.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB1))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE51, Arg2)
                        Return (Zero)
                    }

                    Return (^PE51.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xB0))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE50, Arg2)
                        Return (Zero)
                    }

                    Return (^PE50.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xAF))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE47, Arg2)
                        Return (Zero)
                    }

                    Return (^PE47.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xAE))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE46, Arg2)
                        Return (Zero)
                    }

                    Return (^PE46.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xAD))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE45, Arg2)
                        Return (Zero)
                    }

                    Return (^PE45.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xAC))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE44, Arg2)
                        Return (Zero)
                    }

                    Return (^PE44.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xAB))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE43, Arg2)
                        Return (Zero)
                    }

                    Return (^PE43.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xAA))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE42, Arg2)
                        Return (Zero)
                    }

                    Return (^PE42.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xA9))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE41, Arg2)
                        Return (Zero)
                    }

                    Return (^PE41.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0xA8))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^PE40, Arg2)
                        Return (Zero)
                    }

                    Return (^PE40.PPHR (Arg1, Arg2))
                }

                If ((Arg0 == 0x88))
                {
                    If ((Arg1 == 0xFFFF))
                    {
                        Notify (^P2P0, Arg2)
                        Return (Zero)
                    }

                    Return (^P2P0.PPHR (Arg1, Arg2))
                }

                Return (0xFFFFFFFD)
            }
        }

        Method (IVOC, 2, NotSerialized)
        {
            ^PCI0.OEMR = ((0x40E90000 | (Arg0 << 0x08)) | Arg1)
        }

        Method (VMPS, 1, NotSerialized)
        {
            Acquire (OEML, 0xFFFF)
            IVOC (0x81, Arg0)
            Local0 = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Release (OEML)
            Return (Local0)
        }

        Method (VMBB, 1, NotSerialized)
        {
            Acquire (OEML, 0xFFFF)
            IVOC (0x82, Arg0)
            Local0 = Package (0x0D)
                {
                    Zero, 
                    Zero, 
                    Zero, 
                    Zero, 
                    Zero, 
                    Zero, 
                    Zero, 
                    Zero, 
                    Zero, 
                    "VMware Virtual Battery", 
                    "", 
                    "", 
                    ""
                }
            Local0 [Zero] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [One] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x02] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x03] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x04] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x05] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x06] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x07] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x08] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local1 = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Release (OEML)
            If ((Local1 != One))
            {
                Local0 = Package (0x0D)
                    {
                        Zero, 
                        Ones, 
                        Ones, 
                        Zero, 
                        Ones, 
                        Zero, 
                        Zero, 
                        Zero, 
                        Zero, 
                        "", 
                        "", 
                        "", 
                        ""
                    }
            }

            Return (Local0)
        }

        Method (VMBS, 1, NotSerialized)
        {
            Acquire (OEML, 0xFFFF)
            IVOC (0x83, Arg0)
            Local0 = Package (0x04)
                {
                    Zero, 
                    Zero, 
                    Zero, 
                    Zero
                }
            Local0 [Zero] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [One] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x02] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local0 [0x03] = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local1 = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Release (OEML)
            If ((Local1 != One))
            {
                Local0 = Package (0x04)
                    {
                        Zero, 
                        Ones, 
                        Ones, 
                        Ones
                    }
            }

            Return (Local0)
        }

        Method (VMAP, 0, NotSerialized)
        {
            Acquire (OEML, 0xFFFF)
            IVOC (0x84, Zero)
            Local0 = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Local1 = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Release (OEML)
            If ((Local1 != One))
            {
                Local0 = One
            }

            Return (Local0)
        }

        Device (BAT1)
        {
            Name (_HID, EisaId ("PNP0C0A") /* Control Method Battery */)  // _HID: Hardware ID
            Name (_UID, One)  // _UID: Unique ID
            Name (_PCL, Package (0x01)  // _PCL: Power Consumer List
            {
                _SB
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (VMPS (One))
            }

            Method (_BIF, 0, NotSerialized)  // _BIF: Battery Information
            {
                Return (VMBB (One))
            }

            Method (_BST, 0, NotSerialized)  // _BST: Battery Status
            {
                Return (VMBS (One))
            }

            Method (_INI, 0, NotSerialized)  // _INI: Initialize
            {
                Acquire (OEML, 0xFFFF)
                IVOC (0x87, Zero)
                IVOC (0x88, Zero)
                Release (OEML)
            }
        }

        Device (BAT2)
        {
            Name (_HID, EisaId ("PNP0C0A") /* Control Method Battery */)  // _HID: Hardware ID
            Name (_UID, 0x02)  // _UID: Unique ID
            Name (_PCL, Package (0x01)  // _PCL: Power Consumer List
            {
                _SB
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (VMPS (0x02))
            }

            Method (_BIF, 0, NotSerialized)  // _BIF: Battery Information
            {
                Return (VMBB (0x02))
            }

            Method (_BST, 0, NotSerialized)  // _BST: Battery Status
            {
                Return (VMBS (0x02))
            }
        }

        Device (ACAD)
        {
            Name (_HID, "ACPI0003" /* Power Source Device */)  // _HID: Hardware ID
            Name (_UID, One)  // _UID: Unique ID
            Name (_PCL, Package (0x01)  // _PCL: Power Consumer List
            {
                _SB
            })
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (0x0F)
            }

            Method (_PSR, 0, NotSerialized)  // _PSR: Power Source
            {
                Return (VMAP ())
            }
        }

        Method (_SWS, 0, NotSerialized)  // _SWS: System Wake Source
        {
            Return (0x08)
        }

        Method (BFEA, 0, NotSerialized)
        {
            Acquire (OEML, 0xFFFF)
            IVOC (0x89, Zero)
            Local0 = ^PCI0.OEMR /* \_SB_.PCI0.OEMR */
            Release (OEML)
            Return (Local0)
        }

        Device (SLPB)
        {
            Name (_HID, "PNP0C0E" /* Sleep Button Device */)  // _HID: Hardware ID
            Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
            {
                0x09, 
                0x04
            })
            Method (_PSW, 1, NotSerialized)  // _PSW: Power State Wake
            {
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If ((BFEA () & One))
                {
                    Return (0x0F)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }

        OperationRegion (EREG, SystemMemory, ECFG, 0x0002C000)
        Field (EREG, DWordAcc, NoLock, Preserve)
        {
            Offset (0xA0), 
            DCFI,   32, 
            DSLI,   32, 
            Offset (0xB0), 
            HPST,   32, 
            HPCM,   32, 
            HPSL,   32, 
            Offset (0x100), 
            QSTA,   32, 
            QCMD,   32, 
            QQUA,   32, 
            QDAT,   32, 
            Offset (0x232), 
            ACKW,   32, 
            Offset (0x400), 
            VVTB,   32, 
            VVTS,   32, 
            GENL,   32, 
            GENH,   32, 
            DBDF,   32, 
            DNAM,   256, 
            Offset (0x450), 
            CPUB,   32, 
            Offset (0x458), 
            PCST,   32, 
            Offset (0x500), 
            AIOB,   32, 
            AIOS,   32, 
            EMNA,   64, 
            ELNG,   64, 
            ELNX,   64, 
            EPCS,   8, 
            Offset (0x3004), 
            LDST,   1, 
            Offset (0x3008)
        }

        Field (EREG, AnyAcc, NoLock, Preserve)
        {
            Offset (0x460), 
            TBAS,   32, 
            TPRS,   32, 
            TPOP,   8, 
            TPRQ,   1, 
            Offset (0x46A), 
                ,   1, 
            TP17,   1, 
            TP26,   1, 
            TP28,   1, 
            TP30,   1, 
            TP32,   1, 
            TP98,   1, 
            T100,   1, 
            Offset (0x470), 
                ,   7, 
            TMIM,   1, 
            TMOR,   8
        }

        Processor (CP00, 0x00, 0x00000450, 0x06)
        {
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (CSTA (Zero))
            }

            Method (_MAT, 0, NotSerialized)  // _MAT: Multiple APIC Table Entry
            {
                Return (CMAT (Zero))
            }

            Method (_PXM, 0, NotSerialized)  // _PXM: Device Proximity
            {
                Return (CPXM (Zero))
            }
        }

        Method (CSTA, 1, Serialized)
        {
            If (!Arg0)
            {
                Return (0x0F)
            }

            If ((ECFP & One))
            {
                Local0 = 0x00028800
            }
            Else
            {
                Local0 = 0x40
            }

            Local1 = ((((Arg0 >> 0x05) << 0x02) + Local0) + 
                ECFG) /* \_SB_.ECFG */
            Local3 = (Arg0 & 0x1F)
            OperationRegion (CREG, SystemMemory, Local1, 0x04)
            Field (CREG, DWordAcc, NoLock, Preserve)
            {
                CPUP,   32
            }

            Local0 = CPUP /* \_SB_.CSTA.CPUP */
            If (((One << Local3) & Local0))
            {
                Return (0x0F)
            }

            Return (Zero)
        }

        Method (APID, 1, Serialized)
        {
            If ((ECFP & One))
            {
                Local1 = ((Arg0 << One) + 0x00029000)
            }
            Else
            {
                Local1 = (Arg0 + 0x3200)
            }

            OperationRegion (CREG, SystemMemory, (Local1 + ECFG), 0x02)
            Field (CREG, AnyAcc, NoLock, Preserve)
            {
                LOC0,   16
            }

            If ((ECFP & One))
            {
                Return (LOC0) /* \_SB_.APID.LOC0 */
            }

            Local0 = (LOC0 & 0xFF)
            If (Local0)
            {
                Return (Local0)
            }

            Return (Arg0)
        }

        Method (CMAT, 1, NotSerialized)
        {
            Local2 = APID (Arg0)
            Local3 = (CSTA (Arg0) & One)
            If ((Local2 >= 0xFF))
            {
                Local0 = Buffer (0x10) {}
                CreateByteField (Local0, Zero, TYPE)
                CreateByteField (Local0, One, LEN)
                CreateWordField (Local0, 0x02, RSVD)
                CreateDWordField (Local0, 0x04, X2ID)
                CreateDWordField (Local0, 0x08, FLAG)
                CreateDWordField (Local0, 0x0C, CPID)
                TYPE = 0x09
                LEN = 0x10
                X2ID = Local2
                FLAG = Local3
                CPID = Arg0
            }
            Else
            {
                Local0 = Buffer (0x08)
                    {
                         0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00   // ........
                    }
                Local0 [0x02] = Arg0
                Local0 [0x03] = Local2
                Local0 [0x04] = Local3
            }

            Return (Local0)
        }

        Method (CPXM, 1, Serialized)
        {
            If ((ECFP & One))
            {
                Local0 = (0x00028000 + Arg0)
            }
            Else
            {
                Local0 = (0x3100 + APID (Arg0))
            }

            Local1 = (Local0 + ECFG) /* \_SB_.ECFG */
            OperationRegion (CREG, SystemMemory, Local1, One)
            Field (CREG, ByteAcc, NoLock, Preserve)
            {
                PXMI,   8
            }

            Return (PXMI) /* \_SB_.CPXM.PXMI */
        }

        Method (CEJ0, 2, Serialized)
        {
            If ((ECFP & One))
            {
                CPUB = Arg0
            }
            Else
            {
                Local1 = ((((Arg0 >> 0x05) << 0x02) + 0x60) + 
                    ECFG) /* \_SB_.ECFG */
                Local3 = (Arg0 & 0x1F)
                OperationRegion (CREG, SystemMemory, Local1, 0x04)
                Field (CREG, DWordAcc, NoLock, Preserve)
                {
                    CPUA,   32
                }

                CPUA = (One << Local3)
            }
        }

        Device (EPC)
        {
            Name (_HID, EisaId ("INT0E0C"))  // _HID: Hardware ID
            Name (_STR, Unicode ("Enclave Page Cache 1.0"))  // _STR: Description String
            Name (_MLS, Package (0x01)  // _MLS: Multiple Language String
            {
                Package (0x02)
                {
                    "en", 
                    Unicode ("Enclave Page Cache 1.0")
                }
            })
            Name (RBUF, ResourceTemplate ()
            {
                QWordMemory (ResourceConsumer, PosDecode, MinNotFixed, MaxNotFixed, NonCacheable, ReadWrite,
                    0x0000000000000000, // Granularity
                    0x0000000000000000, // Range Minimum
                    0x0000000000000000, // Range Maximum
                    0x0000000000000000, // Translation Offset
                    0x0000000000000001, // Length
                    ,, _Y07, AddressRangeMemory, TypeStatic)
            })
            Method (_CRS, 0, NotSerialized)  // _CRS: Current Resource Settings
            {
                CreateField (RBUF, \_SB.EPC._Y07._MIN, 0x40, EMIN)  // _MIN: Minimum Base Address
                CreateField (RBUF, \_SB.EPC._Y07._MAX, 0x40, EMAX)  // _MAX: Maximum Base Address
                CreateField (RBUF, \_SB.EPC._Y07._LEN, 0x40, ELEN)  // _LEN: Length
                EMIN = EMNA /* \_SB_.EMNA */
                ELEN = ELNG /* \_SB_.ELNG */
                EMAX = ELNX /* \_SB_.ELNX */
                Return (RBUF) /* \_SB_.EPC_.RBUF */
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If ((EPCS != Zero))
                {
                    Return (0x0F)
                }

                Return (Zero)
            }
        }

        Method (PBAD, 1, NotSerialized)
        {
            If ((Arg0 & 0xFFFF0000))
            {
                Local3 = (Arg0 & 0x07)
                If (Local3)
                {
                    Local3 = ((Local3 << 0x10) | ((Arg0 & 0xFFFF0000) << 0x03
                        ))
                }
                Else
                {
                    Local3 = (Arg0 - 0x00110000)
                }
            }
            Else
            {
                Local3 = (Arg0 << 0x10)
            }

            Return (Local3)
        }

        Method (PEJ0, 3, NotSerialized)
        {
            DSLI = (PBAD (Arg2) | (Arg1 >> 0x10))
        }

        Method (PSTA, 1, NotSerialized)
        {
            If ((Arg0 == 0xFFFF))
            {
                Return (Zero)
            }

            Return (0x0F)
        }

        Name (DNMP, Ones)
        Mutex (DVNL, 0x0F)
        Method (PDSM, 6, Serialized)
        {
            If ((Arg0 == ToUUID ("e5c937d0-3553-4d7a-9117-ea4d19c3434d") /* Device Labeling Interface */))
            {
                If ((Arg1 >= 0x02))
                {
                    If ((Zero == Arg2))
                    {
                        If ((DNMP == Ones))
                        {
                            Acquire (DVNL, 0xFFFF)
                            DBDF = Zero
                            DNMP = DBDF /* \_SB_.DBDF */
                            Release (DVNL)
                        }

                        If (DNMP)
                        {
                            Return (Buffer (One)
                            {
                                 0x81                                             // .
                            })
                        }

                        Return (Buffer (One)
                        {
                             0x01                                             // .
                        })
                    }
                    ElseIf ((0x07 == Arg2))
                    {
                        Local1 = (PBAD (Arg5) | (Arg4 >> 0x08))
                        Local0 = Package (0x02) {}
                        Acquire (DVNL, 0xFFFF)
                        DBDF = Local1
                        Local1 = DBDF /* \_SB_.DBDF */
                        DNMP = Local1
                        If (!Local1)
                        {
                            Release (DVNL)
                            Return (Buffer (One)
                            {
                                 0x00                                             // .
                            })
                        }

                        Local0 [Zero] = Local1
                        Local5 = DNAM /* \_SB_.DNAM */
                        Local4 = Zero
                        While ((DerefOf (Local5 [Local4]) || DerefOf (Local5 [(Local4 + 
                            One)])))
                        {
                            Local4 += 0x02
                        }

                        Local4 += 0x02
                        Local0 [One] = Mid (Local5, Zero, Local4)
                        Release (DVNL)
                        Return (Local0)
                    }
                }
            }

            Return (Buffer (One)
            {
                 0x00                                             // .
            })
        }

        Method (SOSC, 4, Serialized)
        {
            If ((Arg0 == ToUUID ("33db4d5b-1ff7-401c-9657-7441c03dd766") /* PCI Host Bridge Device */))
            {
                CreateDWordField (Arg3, Zero, CDW1)
                CreateDWordField (Arg3, 0x08, CDW3)
                Local0 = (CDW3 & 0x78)
                If (!(POSC & 0x08))
                {
                    Local0 &= 0x50
                }

                If ((Arg1 != One))
                {
                    CDW1 |= 0x08
                }

                If ((CDW3 != Local0))
                {
                    CDW1 |= 0x10
                }

                CDW3 = Local0
            }
            Else
            {
                CDW1 |= 0x04
            }

            Return (Arg3)
        }

        Method (AWAK, 0, Serialized)
        {
            ACKW = Ones
        }

        Device (LID)
        {
            Name (_HID, "PNP0C0D" /* Lid Device */)  // _HID: Hardware ID
            Name (_PRW, Package (0x02)  // _PRW: Power Resources for Wake
            {
                0x09, 
                0x04
            })
            Method (_PSW, 1, NotSerialized)  // _PSW: Power State Wake
            {
            }

            Method (_LID, 0, NotSerialized)  // _LID: Lid Status
            {
                Return (LDST) /* \_SB_.LDST */
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If ((BFEA () & 0x04))
                {
                    Return (0x0F)
                }
                Else
                {
                    Return (Zero)
                }
            }
        }

        Method (DQUE, 0, Serialized)
        {
            Local0 = QCMD /* \_SB_.QCMD */
            Local1 = QQUA /* \_SB_.QQUA */
            Local2 = QDAT /* \_SB_.QDAT */
            If ((Local0 == One))
            {
                QSTA = L1MX (Local2, Local1)
            }
            ElseIf ((Local0 == 0x02))
            {
                QSTA = CNOT (Local1, Local2)
            }
            ElseIf ((Local0 == 0x03))
            {
                If ((Local1 == One))
                {
                    Notify (\_SB.SLPB, 0x80) // Status Change
                    QSTA = One
                }
                ElseIf ((Local1 == 0x03))
                {
                    Notify (\_SB.LID, 0x80) // Status Change
                    QSTA = 0x03
                }
                Else
                {
                    QSTA = 0xFFFFFFFE
                }
            }
            ElseIf ((Local0 == 0x04))
            {
                Notify (\_SB.PCI0.VMGC, 0x80) // Status Change
                QSTA = One
            }
            ElseIf ((Local0 == 0x05))
            {
                If ((Local1 == Zero))
                {
                    Notify (\_SB.NVD.NV00, QDAT)
                }
                ElseIf ((Local1 == One))
                {
                    Notify (\_SB.NVD.NV01, QDAT)
                }
                ElseIf ((Local1 == 0x02))
                {
                    Notify (\_SB.NVD.NV02, QDAT)
                }
                ElseIf ((Local1 == 0x03))
                {
                    Notify (\_SB.NVD.NV03, QDAT)
                }
                ElseIf ((Local1 == 0x04))
                {
                    Notify (\_SB.NVD.NV04, QDAT)
                }
                ElseIf ((Local1 == 0x05))
                {
                    Notify (\_SB.NVD.NV05, QDAT)
                }
                ElseIf ((Local1 == 0x06))
                {
                    Notify (\_SB.NVD.NV06, QDAT)
                }
                ElseIf ((Local1 == 0x07))
                {
                    Notify (\_SB.NVD.NV07, QDAT)
                }
                ElseIf ((Local1 == 0x08))
                {
                    Notify (\_SB.NVD.NV08, QDAT)
                }
                ElseIf ((Local1 == 0x09))
                {
                    Notify (\_SB.NVD.NV09, QDAT)
                }
                ElseIf ((Local1 == 0x0A))
                {
                    Notify (\_SB.NVD.NV10, QDAT)
                }
                ElseIf ((Local1 == 0x0B))
                {
                    Notify (\_SB.NVD.NV11, QDAT)
                }
                ElseIf ((Local1 == 0x0C))
                {
                    Notify (\_SB.NVD.NV12, QDAT)
                }
                ElseIf ((Local1 == 0x0D))
                {
                    Notify (\_SB.NVD.NV13, QDAT)
                }
                ElseIf ((Local1 == 0x0E))
                {
                    Notify (\_SB.NVD.NV14, QDAT)
                }
                ElseIf ((Local1 == 0x0F))
                {
                    Notify (\_SB.NVD.NV15, QDAT)
                }
                ElseIf ((Local1 == 0x10))
                {
                    Notify (\_SB.NVD.NV16, QDAT)
                }
                ElseIf ((Local1 == 0x11))
                {
                    Notify (\_SB.NVD.NV17, QDAT)
                }
                ElseIf ((Local1 == 0x12))
                {
                    Notify (\_SB.NVD.NV18, QDAT)
                }
                ElseIf ((Local1 == 0x13))
                {
                    Notify (\_SB.NVD.NV19, QDAT)
                }
                ElseIf ((Local1 == 0x14))
                {
                    Notify (\_SB.NVD.NV20, QDAT)
                }
                ElseIf ((Local1 == 0x15))
                {
                    Notify (\_SB.NVD.NV21, QDAT)
                }
                ElseIf ((Local1 == 0x16))
                {
                    Notify (\_SB.NVD.NV22, QDAT)
                }
                ElseIf ((Local1 == 0x17))
                {
                    Notify (\_SB.NVD.NV23, QDAT)
                }
                ElseIf ((Local1 == 0x18))
                {
                    Notify (\_SB.NVD.NV24, QDAT)
                }
                ElseIf ((Local1 == 0x19))
                {
                    Notify (\_SB.NVD.NV25, QDAT)
                }
                ElseIf ((Local1 == 0x1A))
                {
                    Notify (\_SB.NVD.NV26, QDAT)
                }
                ElseIf ((Local1 == 0x1B))
                {
                    Notify (\_SB.NVD.NV27, QDAT)
                }
                ElseIf ((Local1 == 0x1C))
                {
                    Notify (\_SB.NVD.NV28, QDAT)
                }
                ElseIf ((Local1 == 0x1D))
                {
                    Notify (\_SB.NVD.NV29, QDAT)
                }
                ElseIf ((Local1 == 0x1E))
                {
                    Notify (\_SB.NVD.NV30, QDAT)
                }
                ElseIf ((Local1 == 0x1F))
                {
                    Notify (\_SB.NVD.NV31, QDAT)
                }
                ElseIf ((Local1 == 0x20))
                {
                    Notify (\_SB.NVD.NV32, QDAT)
                }
                ElseIf ((Local1 == 0x21))
                {
                    Notify (\_SB.NVD.NV33, QDAT)
                }
                ElseIf ((Local1 == 0x22))
                {
                    Notify (\_SB.NVD.NV34, QDAT)
                }
                ElseIf ((Local1 == 0x23))
                {
                    Notify (\_SB.NVD.NV35, QDAT)
                }
                ElseIf ((Local1 == 0x24))
                {
                    Notify (\_SB.NVD.NV36, QDAT)
                }
                ElseIf ((Local1 == 0x25))
                {
                    Notify (\_SB.NVD.NV37, QDAT)
                }
                ElseIf ((Local1 == 0x26))
                {
                    Notify (\_SB.NVD.NV38, QDAT)
                }
                ElseIf ((Local1 == 0x27))
                {
                    Notify (\_SB.NVD.NV39, QDAT)
                }
                ElseIf ((Local1 == 0x28))
                {
                    Notify (\_SB.NVD.NV40, QDAT)
                }
                ElseIf ((Local1 == 0x29))
                {
                    Notify (\_SB.NVD.NV41, QDAT)
                }
                ElseIf ((Local1 == 0x2A))
                {
                    Notify (\_SB.NVD.NV42, QDAT)
                }
                ElseIf ((Local1 == 0x2B))
                {
                    Notify (\_SB.NVD.NV43, QDAT)
                }
                ElseIf ((Local1 == 0x2C))
                {
                    Notify (\_SB.NVD.NV44, QDAT)
                }
                ElseIf ((Local1 == 0x2D))
                {
                    Notify (\_SB.NVD.NV45, QDAT)
                }
                ElseIf ((Local1 == 0x2E))
                {
                    Notify (\_SB.NVD.NV46, QDAT)
                }
                ElseIf ((Local1 == 0x2F))
                {
                    Notify (\_SB.NVD.NV47, QDAT)
                }
                ElseIf ((Local1 == 0x30))
                {
                    Notify (\_SB.NVD.NV48, QDAT)
                }
                ElseIf ((Local1 == 0x31))
                {
                    Notify (\_SB.NVD.NV49, QDAT)
                }
                ElseIf ((Local1 == 0x32))
                {
                    Notify (\_SB.NVD.NV50, QDAT)
                }
                ElseIf ((Local1 == 0x33))
                {
                    Notify (\_SB.NVD.NV51, QDAT)
                }
                ElseIf ((Local1 == 0x34))
                {
                    Notify (\_SB.NVD.NV52, QDAT)
                }
                ElseIf ((Local1 == 0x35))
                {
                    Notify (\_SB.NVD.NV53, QDAT)
                }
                ElseIf ((Local1 == 0x36))
                {
                    Notify (\_SB.NVD.NV54, QDAT)
                }
                ElseIf ((Local1 == 0x37))
                {
                    Notify (\_SB.NVD.NV55, QDAT)
                }
                ElseIf ((Local1 == 0x38))
                {
                    Notify (\_SB.NVD.NV56, QDAT)
                }
                ElseIf ((Local1 == 0x39))
                {
                    Notify (\_SB.NVD.NV57, QDAT)
                }
                ElseIf ((Local1 == 0x3A))
                {
                    Notify (\_SB.NVD.NV58, QDAT)
                }
                ElseIf ((Local1 == 0x3B))
                {
                    Notify (\_SB.NVD.NV59, QDAT)
                }
                ElseIf ((Local1 == 0x3C))
                {
                    Notify (\_SB.NVD.NV60, QDAT)
                }
                ElseIf ((Local1 == 0x3D))
                {
                    Notify (\_SB.NVD.NV61, QDAT)
                }
                ElseIf ((Local1 == 0x3E))
                {
                    Notify (\_SB.NVD.NV62, QDAT)
                }
                ElseIf ((Local1 == 0x3F))
                {
                    Notify (\_SB.NVD.NV63, QDAT)
                }

                QSTA = One
            }
            Else
            {
                QSTA = Ones
            }
        }

        Device (NVD)
        {
            Name (_HID, "VMW0002")  // _HID: Hardware ID
            Name (_CID, Package (0x02)  // _CID: Compatible ID
            {
                "ACPI0012" /* NVDIMM Root Device */, 
                EisaId ("PNP0C02") /* PNP Motherboard Resources */
            })
            Name (_UID, Zero)  // _UID: Unique ID
            Method (_INI, 0, Serialized)  // _INI: Initialize
            {
                If ((TOOS == 0x1000))
                {
                    _CID = 0x020CD041
                }
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (NVDM)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Mutex (NVDL, 0x0F)
            Field (EREG, AnyAcc, NoLock, Preserve)
            {
                Offset (0x3C00), 
                FCTL,   32, 
                FIDX,   4096, 
                FLB1,   96, 
                FLB2,   32, 
                Offset (0x3E84), 
                AADR,   64, 
                ALEN,   64, 
                ATYP,   16, 
                ACMD,   8, 
                ARES,   24, 
                ASIZ,   16, 
                AOUT,   576, 
                NLWT,   8, 
                ELWT,   8, 
                ETWT,   8, 
                Offset (0x3F00), 
                FHAB,   64, 
                FICX,   16
            }

            Field (EREG, WordAcc, NoLock, Preserve)
            {
                Offset (0x3F0A), 
                ARSF,   16
            }

            Field (EREG, DWordAcc, NoLock, Preserve)
            {
                Offset (0x3F10), 
                HCMD,   32, 
                HOUT,   512
            }

            Method (FIT1, 1, Serialized)
            {
                Local1 = (Arg0 << 0x04)
                Local1 = ((Local1 + 0x00018000) + ECFG) /* \_SB_.ECFG */
                OperationRegion (BREG, SystemMemory, Local1, 0x10)
                Field (BREG, ByteAcc, NoLock, Preserve)
                {
                    Offset (0x01), 
                    PXM,    8, 
                    RSV1,   6, 
                    BASE,   42, 
                    Offset (0x0A), 
                    RSV2,   6, 
                    SIZE,   42
                }

                Local0 = Buffer (0x80) {}
                Local0 [0x02] = 0x38
                Local0 [0x04] = (Arg0 + One)
                Local0 [0x06] = 0x02
                Local0 [0x0C] = PXM /* \_SB_.NVD_.FIT1.PXM_ */
                CreateField (Local0, 0x80, 0x80, GUID)
                GUID = /**** Is ResourceTemplate, but EndTag not at buffer end ****/ ToUUID ("66f0d379-b4f3-4074-ac43-0d3318b78cdb") /* Persistent Memory Region */
                CreateField (Local0, 0x0116, 0x2A, BAS1)
                BAS1 = BASE /* \_SB_.NVD_.FIT1.BASE */
                CreateField (Local0, 0x0156, 0x2A, SIZ1)
                SIZ1 = SIZE /* \_SB_.NVD_.FIT1.SIZE */
                CreateQWordField (Local0, 0x30, ATTR)
                ATTR = 0x8008
                Local0 [0x38] = One
                Local0 [0x3A] = 0x30
                Local0 [0x3C] = Arg0
                CreateDWordField (Local0, 0x40, SMBI)
                SMBI = (NVDH + Arg0)
                Local0 [0x44] = (Arg0 + One)
                Local0 [0x46] = (Arg0 + One)
                CreateField (Local0, 0x0256, 0x2A, SIZ2)
                SIZ2 = SIZE /* \_SB_.NVD_.FIT1.SIZE */
                Local0 [0x68] = 0x06
                Local0 [0x6A] = 0x18
                Local0 [0x6C] = Arg0
                Local0 [0x70] = One
                CreateQWordField (Local0, 0x78, FHA1)
                FHA1 = FHAB /* \_SB_.NVD_.FHAB */
                Return (Local0)
            }

            Method (CRDT, 1, Serialized)
            {
                Local0 = Buffer (0x20) {}
                Local0 [Zero] = 0x04
                Local0 [0x02] = 0x20
                Local0 [0x04] = (Arg0 + One)
                CreateDWordField (Local0, 0x06, PCID)
                PCID = 0x00010289
                CreateDWordField (Local0, 0x0C, SSID)
                SSID = 0x00010289
                CreateWordField (Local0, 0x1C, FMTC)
                FMTC = FICX /* \_SB_.NVD_.FICX */
                CreateDWordField (Local0, 0x18, SNUM)
                Acquire (NVDL, 0xFFFF)
                FCTL = Arg0
                SNUM = FLB2 /* \_SB_.NVD_.FLB2 */
                Release (NVDL)
                Return (Local0)
            }

            Method (PCS, 0, Serialized)
            {
                Local0 = Buffer (0x10) {}
                Local0 [Zero] = 0x07
                Local0 [0x02] = 0x10
                Local0 [0x04] = 0x02
                Local0 [0x08] = 0x02
                Return (Local0)
            }

            Method (_FIT, 0, Serialized)  // _FIT: Firmware Interface Table
            {
                If (NVDM)
                {
                    Local1 = 0x40
                    Local2 = Zero
                    While (Local1)
                    {
                        Local1--
                        If ((DSTA (Local1) == 0x0F))
                        {
                            If ((Local2 == Zero))
                            {
                                Local2++
                                Local0 = CRDT (Local1)
                            }
                            Else
                            {
                                Concatenate (CRDT (Local1), Local0, Local0)
                            }

                            Concatenate (FIT1 (Local1), Local0, Local0)
                        }
                    }

                    If ((Local2 != Zero))
                    {
                        Concatenate (PCS (), Local0, Local0)
                    }

                    Return (Local0)
                }

                Return (Buffer (One)
                {
                     0x00                                             // .
                })
            }

            Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
            {
                If ((Arg0 == ToUUID ("2f10e7a4-9e91-11e4-89d3-123b93f75cba") /* NVDIMM Root Device */))
                {
                    If ((Arg1 >= One))
                    {
                        If ((Zero == Arg2))
                        {
                            Local0 = Buffer (0x02) {}
                            CreateWordField (Local0, Zero, ARSW)
                            ARSW = ARSF /* \_SB_.NVD_.ARSF */
                            Return (Local0)
                        }
                        ElseIf ((Arg2 >= 0x05))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x01, 0x00, 0x00, 0x00                           // ....
                            })
                        }
                        ElseIf ((One == Arg2))
                        {
                            Local0 = Buffer (0x10) {}
                            CreateDWordField (Local0, 0x08, CUCE)
                            CUCE = 0x04
                            CreateDWordField (Local0, 0x04, MAXD)
                            MAXD = 0x48
                            Local0 [0x02] = 0x02
                            Local0 [Zero] = Zero
                            Return (Local0)
                        }
                        Else
                        {
                            If ((0x02 == Arg2))
                            {
                                CreateQWordField (DerefOf (Arg3 [Zero]), Zero, SADR)
                                CreateQWordField (DerefOf (Arg3 [Zero]), 0x08, SLEN)
                                CreateWordField (DerefOf (Arg3 [Zero]), 0x10, STYP)
                                AADR = SADR /* \_SB_.NVD_._DSM.SADR */
                                ALEN = SLEN /* \_SB_.NVD_._DSM.SLEN */
                                ATYP = STYP /* \_SB_.NVD_._DSM.STYP */
                                ACMD = 0x02
                            }
                            ElseIf ((0x03 == Arg2))
                            {
                                ACMD = 0x03
                            }
                            Else
                            {
                                CreateQWordField (DerefOf (Arg3 [Zero]), Zero, CADR)
                                CreateQWordField (DerefOf (Arg3 [Zero]), 0x08, CLEN)
                                AADR = CADR /* \_SB_.NVD_._DSM.CADR */
                                ALEN = CLEN /* \_SB_.NVD_._DSM.CLEN */
                                ACMD = 0x04
                            }

                            Local2 = ASIZ /* \_SB_.NVD_.ASIZ */
                            Return (Mid (AOUT, Zero, Local2))
                        }
                    }
                }

                Return (Buffer (One)
                {
                     0x00                                             // .
                })
            }

            Method (PCFG, 2, Serialized)
            {
                CreateDWordField (DerefOf (Arg0 [Zero]), Zero, OFFS)
                CreateDWordField (DerefOf (Arg0 [Zero]), 0x04, OLEN)
                If (((OLEN + OFFS) > 0x00020000))
                {
                    Return (Buffer (0x04)
                    {
                         0x03, 0x00, 0x00, 0x00                           // ....
                    })
                }

                If ((OFFS >= 0x0280))
                {
                    Local2 = (OLEN + 0x04)
                    Local0 = Buffer (Local2) {}
                    Return (Local0)
                }

                Local2 = (OLEN + 0x04)
                Local0 = Buffer (Local2) {}
                If (((OLEN + OFFS) > 0x0280))
                {
                    OLEN = (0x0280 - OFFS) /* \_SB_.NVD_.PCFG.OFFS */
                    Local2 = (OLEN + 0x04)
                }

                Local1 = (0x3C00 + ECFG) /* \_SB_.ECFG */
                OperationRegion (CDAT, SystemMemory, Local1, 0x0284)
                Local2 = (OLEN << 0x03)
                Field (CDAT, AnyAcc, NoLock, Preserve)
                {
                    FCTL,   32, 
                    FDAT,   5120
                }

                CreateField (Local0, 0x20, Local2, OBUF)
                FCTL = Arg1
                OBUF = Mid (FDAT, OFFS, OLEN)
                Local0 [Zero] = Zero
                Return (Local0)
            }

            Method (HDSM, 5, Serialized)
            {
                If ((Zero == Arg2))
                {
                    Return (Unicode ("7"))
                }

                If ((DSTA (Arg4) != 0x0F))
                {
                    Return (Buffer (0x04)
                    {
                         0x02, 0x00, 0x00, 0x00                           // ....
                    })
                }

                If ((One == Arg2))
                {
                    Return (Buffer (0x80)
                    {
                        /* 0000 */  0x00, 0x00, 0x00, 0x00, 0xFF, 0x01, 0x00, 0x00,  // ........
                        /* 0008 */  0x00, 0x40, 0x01, 0x64, 0x00, 0x00, 0x01, 0x02,  // .@.d....
                        /* 0010 */  0x0A, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x00, 0x00,  // ........
                        /* 0018 */  0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,  // ........
                        /* 0020 */  0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,  // ........
                        /* 0028 */  0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,  // ........
                        /* 0030 */  0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                        /* 0038 */  0x00, 0x00, 0x00, 0x00, 0x63, 0x40, 0x01, 0x00,  // ....c@..
                        /* 0040 */  0x01, 0x00, 0x00, 0x00                           // ....
                    })
                }

                If ((0x02 == Arg2))
                {
                    Return (Buffer (0x24) {})
                }

                If ((0x04 == Arg2))
                {
                    Return (Buffer (0x0C)
                    {
                        /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,  // ........
                        /* 0008 */  0x00, 0x00, 0x02, 0x00                           // ....
                    })
                }

                If ((0x05 == Arg2))
                {
                    Return (PCFG (Arg3, Arg4))
                }

                Return (Buffer (0x04)
                {
                     0x01, 0x00, 0x00, 0x00                           // ....
                })
            }

            Method (MDSM, 5, Serialized)
            {
                If ((Zero == Arg2))
                {
                    Return (Buffer (0x04)
                    {
                         0xFF, 0xFF, 0x07, 0x04                           // ....
                    })
                }
                Else
                {
                    If ((DSTA (Arg4) != 0x0F))
                    {
                        Return (Buffer (0x04)
                        {
                             0x03, 0x00, 0x00, 0x00                           // ....
                        })
                    }

                    If ((SizeOf (Arg3) != One))
                    {
                        Return (Buffer (0x04)
                        {
                             0x02, 0x00, 0x00, 0x00                           // ....
                        })
                    }

                    Local0 = DerefOf (Arg3 [Zero])
                    If ((ObjectType (Local0) != 0x03))
                    {
                        Return (Buffer (0x04)
                        {
                             0x02, 0x00, 0x00, 0x00                           // ....
                        })
                    }

                    If ((0x06 == Arg2))
                    {
                        If ((SizeOf (Local0) != One))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x02, 0x00, 0x00, 0x00                           // ....
                            })
                        }

                        CreateByteField (DerefOf (Arg3 [Zero]), Zero, PCT0)
                        If ((PCT0 > 0x64))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x02, 0x00, 0x00, 0x00                           // ....
                            })
                        }

                        Acquire (NVDL, 0xFFFF)
                        FCTL = Arg4
                        NLWT = PCT0 /* \_SB_.NVD_.MDSM.PCT0 */
                        Release (NVDL)
                        Return (Buffer (0x04)
                        {
                             0x00, 0x00, 0x00, 0x00                           // ....
                        })
                    }
                    ElseIf ((0x08 == Arg2))
                    {
                        If ((SizeOf (Local0) != One))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x02, 0x00, 0x00, 0x00                           // ....
                            })
                        }

                        CreateByteField (DerefOf (Arg3 [Zero]), Zero, PCT1)
                        If ((PCT1 > 0x64))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x02, 0x00, 0x00, 0x00                           // ....
                            })
                        }

                        Acquire (NVDL, 0xFFFF)
                        FCTL = Arg4
                        ELWT = PCT1 /* \_SB_.NVD_.MDSM.PCT1 */
                        Release (NVDL)
                        Return (Buffer (0x04)
                        {
                             0x00, 0x00, 0x00, 0x00                           // ....
                        })
                    }
                    ElseIf ((0x09 == Arg2))
                    {
                        If ((SizeOf (Local0) != One))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x02, 0x00, 0x00, 0x00                           // ....
                            })
                        }

                        CreateByteField (DerefOf (Arg3 [Zero]), Zero, PCT2)
                        If ((PCT2 > 0x64))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x02, 0x00, 0x00, 0x00                           // ....
                            })
                        }

                        Acquire (NVDL, 0xFFFF)
                        FCTL = Arg4
                        ETWT = PCT2 /* \_SB_.NVD_.MDSM.PCT2 */
                        Release (NVDL)
                        Return (Buffer (0x04)
                        {
                             0x00, 0x00, 0x00, 0x00                           // ....
                        })
                    }
                    ElseIf ((0x1A == Arg2))
                    {
                        If ((SizeOf (Local0) != One))
                        {
                            Return (Buffer (0x04)
                            {
                                 0x02, 0x00, 0x00, 0x00                           // ....
                            })
                        }

                        Return (Buffer (0x06)
                        {
                             0x00, 0x00, 0x00, 0x00, 0x00, 0x01               // ......
                        })
                    }

                    If ((SizeOf (Local0) != Zero))
                    {
                        Return (Buffer (0x04)
                        {
                             0x02, 0x00, 0x00, 0x00                           // ....
                        })
                    }

                    If ((One == Arg2))
                    {
                        Return (Buffer (0x32)
                        {
                            /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x20, 0x04, 0x04, 0x00,  // .... ...
                            /* 0008 */  0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x02,  // ........
                            /* 0010 */  0x00, 0x01, 0x01, 0x00, 0x0A, 0x80, 0x00, 0x00,  // ........
                            /* 0018 */  0x0A, 0x80, 0x00, 0x00, 0x0A, 0x80, 0x00, 0x00,  // ........
                            /* 0020 */  0x0A, 0x80, 0x00, 0x00, 0x0A, 0x80, 0x00, 0x00,  // ........
                            /* 0028 */  0x0A, 0x80, 0x00, 0x00, 0x00, 0x64, 0x08, 0x00,  // .....d..
                            /* 0030 */  0x00, 0x00                                       // ..
                        })
                    }
                    ElseIf ((0x02 == Arg2))
                    {
                        Return (Buffer (0x0C)
                        {
                            /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x64, 0x00, 0x64, 0x00,  // ....d.d.
                            /* 0008 */  0x64, 0x00, 0xC8, 0x00                           // d...
                        })
                    }
                    ElseIf ((0x03 == Arg2))
                    {
                        Return (Buffer (0x13)
                        {
                            /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0010 */  0x00, 0x01, 0x04                                 // ...
                        })
                    }
                    ElseIf ((0x04 == Arg2))
                    {
                        Return (Buffer (0x0C)
                        {
                            /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x00, 0x00, 0x00, 0x00                           // ....
                        })
                    }
                    ElseIf ((0x05 == Arg2))
                    {
                        Acquire (NVDL, 0xFFFF)
                        FCTL = Arg4
                        Local3 = Buffer (0x06) {}
                        CreateByteField (Local3, 0x04, RNWT)
                        CreateByteField (Local3, 0x05, RNET)
                        RNWT = NLWT /* \_SB_.NVD_.NLWT */
                        RNET = One
                        Release (NVDL)
                        Return (Local3)
                    }
                    ElseIf ((0x07 == Arg2))
                    {
                        Acquire (NVDL, 0xFFFF)
                        FCTL = Arg4
                        Local3 = Buffer (0x08) {}
                        CreateByteField (Local3, 0x04, REWT)
                        REWT = ELWT /* \_SB_.NVD_.ELWT */
                        CreateByteField (Local3, 0x06, RTWT)
                        RTWT = ETWT /* \_SB_.NVD_.ETWT */
                        Release (NVDL)
                        Return (Local3)
                    }
                    ElseIf ((0x0A == Arg2))
                    {
                        Return (Buffer (0x05)
                        {
                             0x00, 0x00, 0x00, 0x00, 0x00                     // .....
                        })
                    }
                    ElseIf ((0x0B == Arg2))
                    {
                        Return (Buffer (0x0D)
                        {
                            /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x00,  // ........
                            /* 0008 */  0x00, 0x00, 0x64, 0x00, 0x00                     // ..d..
                        })
                    }
                    ElseIf ((0x0C == Arg2))
                    {
                        Return (Buffer (0x0B)
                        {
                            /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x64, 0x50, 0x01, 0x00,  // ....dP..
                            /* 0008 */  0x00, 0x00, 0x00                                 // ...
                        })
                    }
                    ElseIf ((0x0D == Arg2))
                    {
                        Return (Buffer (0x20)
                        {
                            /* 0000 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0008 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0010 */  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // ........
                            /* 0018 */  0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00   // ........
                        })
                    }
                    ElseIf ((0x0E == Arg2))
                    {
                        Return (Buffer (0x08) {})
                    }
                    ElseIf ((0x0F == Arg2))
                    {
                        Return (Buffer (0x04) {})
                    }
                    ElseIf ((0x10 == Arg2))
                    {
                        Return (Buffer (0x05) {})
                    }
                    ElseIf ((0x11 == Arg2))
                    {
                        Return (Buffer (0x04)
                        {
                             0x04, 0x01, 0x00, 0x00                           // ....
                        })
                    }
                    ElseIf ((0x12 == Arg2))
                    {
                        Return (Buffer (0x08) {})
                    }
                    Else
                    {
                        Return (Buffer (0x04)
                        {
                             0x01, 0x00, 0x00, 0x00                           // ....
                        })
                    }
                }
            }

            Method (DNXX, 2, Serialized)
            {
                Local0 = Buffer (0x40) {}
                If ((HCMD != Zero))
                {
                    HCMD = ((Arg0 << 0x10) | Arg1)
                    Local0 = HOUT /* \_SB_.NVD_.HOUT */
                }
                Else
                {
                    CreateWordField (Local0, Zero, NSTS)
                    NSTS = One
                }

                Return (Local0)
            }

            Method (DDSM, 5, Serialized)
            {
                If ((Arg0 == ToUUID ("4309ac30-0d11-11e4-9191-0800200c9a66") /* Unknown UUID */))
                {
                    If (((Arg1 >= One) && (FICX != 0x0101)))
                    {
                        If ((Zero == Arg2))
                        {
                            Return (Buffer (One)
                            {
                                 0x33                                             // 3
                            })
                        }
                        Else
                        {
                            If ((Arg2 >= 0x06))
                            {
                                Return (Buffer (0x04)
                                {
                                     0x01, 0x00, 0x00, 0x00                           // ....
                                })
                            }
                            ElseIf ((0x02 == Arg2))
                            {
                                Return (Buffer (0x04)
                                {
                                     0x01, 0x00, 0x00, 0x00                           // ....
                                })
                            }
                            ElseIf ((0x03 == Arg2))
                            {
                                Return (Buffer (0x04)
                                {
                                     0x01, 0x00, 0x00, 0x00                           // ....
                                })
                            }

                            If ((DSTA (Arg4) != 0x0F))
                            {
                                Return (Buffer (0x04)
                                {
                                     0x02, 0x00, 0x00, 0x00                           // ....
                                })
                            }

                            If ((One == Arg2))
                            {
                                Local0 = Buffer (0x84) {}
                                Local1 = (Arg4 << 0x04)
                                Local1 = ((Local1 + 0x00018000) + ECFG) /* \_SB_.ECFG */
                                OperationRegion (BREG, SystemMemory, Local1, 0x10)
                                Field (BREG, ByteAcc, NoLock, Preserve)
                                {
                                    Offset (0x08), 
                                    HLTH,   8
                                }

                                Local0 [0x04] = One
                                CreateDWordField (Local0, 0x08, RSVD)
                                RSVD = Zero
                                Local0 [0x0C] = HLTH /* \_SB_.NVD_.DDSM.HLTH */
                                Local0 [0x0F] = Zero
                                CreateDWordField (Local0, 0x10, VSDS)
                                VSDS = Zero
                                Local0 [Zero] = Zero
                                Return (Local0)
                            }
                            ElseIf ((0x04 == Arg2))
                            {
                                Local0 = Buffer (0x0C) {}
                                CreateDWordField (Local0, 0x04, CFGS)
                                CFGS = 0x00020000
                                CreateDWordField (Local0, 0x08, MAXS)
                                MAXS = 0x00020000
                                Local0 [Zero] = Zero
                                Return (Local0)
                            }
                            Else
                            {
                                Return (PCFG (Arg3, Arg4))
                            }
                        }
                    }
                }
                ElseIf ((Arg0 == ToUUID ("1ee68b36-d4bd-4a1a-9a16-4f8e53d46e05") /* Unknown UUID */))
                {
                    If (((Arg1 >= One) && (FICX == 0x0101)))
                    {
                        Return (MDSM (Arg0, Arg1, Arg2, Arg3, Arg4))
                    }
                }
                ElseIf ((Arg0 == ToUUID ("9002c334-acf3-4c0e-9642-a235f0d53bc6") /* Unknown UUID */))
                {
                    If (((Arg1 >= One) && (FICX == 0x0101)))
                    {
                        Return (HDSM (Arg0, Arg1, Arg2, Arg3, Arg4))
                    }
                }

                Return (Buffer (One)
                {
                     0x00                                             // .
                })
            }

            Method (DLSI, 0, Serialized)
            {
                Return (Package (0x03)
                {
                    Zero, 
                    0x00020000, 
                    0x00020000
                })
            }

            Method (DLSR, 3, Serialized)
            {
                If (((Arg0 + Arg1) > 0x00020000))
                {
                    Return (Package (0x02)
                    {
                        0x02, 
                        Buffer (One)
                        {
                             0x00                                             // .
                        }
                    })
                }

                Local0 = Package (0x02) {}
                If ((Arg0 >= 0x0280))
                {
                    Local0 [Zero] = Zero
                    Local0 [One] = Buffer (Arg1) {}
                    Return (Local0)
                }

                Local1 = Buffer (Arg1) {}
                If (((Arg0 + Arg1) > 0x0280))
                {
                    Arg1 = (0x0280 - Arg0)
                }

                Local2 = (0x3C00 + ECFG) /* \_SB_.ECFG */
                OperationRegion (CDAT, SystemMemory, Local2, 0x0284)
                Local3 = (Arg1 << 0x03)
                Field (CDAT, AnyAcc, NoLock, Preserve)
                {
                    FCTL,   32, 
                    FDAT,   5120
                }

                CreateField (Local1, Zero, Local3, OBUF)
                FCTL = Arg2
                OBUF = Mid (FDAT, Arg0, Arg1)
                Local0 [Zero] = Zero
                Local0 [One] = Local1
                Return (Local0)
            }

            Method (DLSW, 0, Serialized)
            {
                Return (0x03)
            }

            Method (DSTA, 1, Serialized)
            {
                Local0 = (Arg0 << 0x04)
                Local1 = ((Local0 + 0x00018000) + ECFG) /* \_SB_.ECFG */
                OperationRegion (BREG, SystemMemory, Local1, One)
                Field (BREG, ByteAcc, NoLock, Preserve)
                {
                    LOC2,   1
                }

                Local2 = LOC2 /* \_SB_.NVD_.DSTA.LOC2 */
                If (Local2)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Device (NV00)
            {
                Name (_ADR, Zero)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, Zero))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (Zero, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (Zero, 0x02))
                }
            }

            Device (NV01)
            {
                Name (_ADR, One)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, One))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (One, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (One, 0x02))
                }
            }

            Device (NV02)
            {
                Name (_ADR, 0x02)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x02))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x02, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x02, 0x02))
                }
            }

            Device (NV03)
            {
                Name (_ADR, 0x03)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x03))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x03, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x03, 0x02))
                }
            }

            Device (NV04)
            {
                Name (_ADR, 0x04)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x04))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x04, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x04, 0x02))
                }
            }

            Device (NV05)
            {
                Name (_ADR, 0x05)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x05))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x05, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x05, 0x02))
                }
            }

            Device (NV06)
            {
                Name (_ADR, 0x06)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x06))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x06, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x06, 0x02))
                }
            }

            Device (NV07)
            {
                Name (_ADR, 0x07)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x07))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x07, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x07, 0x02))
                }
            }

            Device (NV08)
            {
                Name (_ADR, 0x08)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x08))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x08, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x08, 0x02))
                }
            }

            Device (NV09)
            {
                Name (_ADR, 0x09)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x09))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x09, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x09, 0x02))
                }
            }

            Device (NV10)
            {
                Name (_ADR, 0x0A)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x0A))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x0A, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x0A, 0x02))
                }
            }

            Device (NV11)
            {
                Name (_ADR, 0x0B)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x0B))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x0B, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x0B, 0x02))
                }
            }

            Device (NV12)
            {
                Name (_ADR, 0x0C)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x0C))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x0C, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x0C, 0x02))
                }
            }

            Device (NV13)
            {
                Name (_ADR, 0x0D)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x0D))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x0D, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x0D, 0x02))
                }
            }

            Device (NV14)
            {
                Name (_ADR, 0x0E)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x0E))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x0E, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x0E, 0x02))
                }
            }

            Device (NV15)
            {
                Name (_ADR, 0x0F)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x0F))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x0F, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x0F, 0x02))
                }
            }

            Device (NV16)
            {
                Name (_ADR, 0x10)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x10))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x10, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x10, 0x02))
                }
            }

            Device (NV17)
            {
                Name (_ADR, 0x11)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x11))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x11, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x11, 0x02))
                }
            }

            Device (NV18)
            {
                Name (_ADR, 0x12)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x12))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x12, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x12, 0x02))
                }
            }

            Device (NV19)
            {
                Name (_ADR, 0x13)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x13))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x13, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x13, 0x02))
                }
            }

            Device (NV20)
            {
                Name (_ADR, 0x14)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x14))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x14, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x14, 0x02))
                }
            }

            Device (NV21)
            {
                Name (_ADR, 0x15)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x15))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x15, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x15, 0x02))
                }
            }

            Device (NV22)
            {
                Name (_ADR, 0x16)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x16))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x16, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x16, 0x02))
                }
            }

            Device (NV23)
            {
                Name (_ADR, 0x17)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x17))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x17, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x17, 0x02))
                }
            }

            Device (NV24)
            {
                Name (_ADR, 0x18)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x18))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x18, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x18, 0x02))
                }
            }

            Device (NV25)
            {
                Name (_ADR, 0x19)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x19))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x19, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x19, 0x02))
                }
            }

            Device (NV26)
            {
                Name (_ADR, 0x1A)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x1A))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x1A, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x1A, 0x02))
                }
            }

            Device (NV27)
            {
                Name (_ADR, 0x1B)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x1B))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x1B, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x1B, 0x02))
                }
            }

            Device (NV28)
            {
                Name (_ADR, 0x1C)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x1C))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x1C, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x1C, 0x02))
                }
            }

            Device (NV29)
            {
                Name (_ADR, 0x1D)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x1D))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x1D, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x1D, 0x02))
                }
            }

            Device (NV30)
            {
                Name (_ADR, 0x1E)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x1E))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x1E, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x1E, 0x02))
                }
            }

            Device (NV31)
            {
                Name (_ADR, 0x1F)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x1F))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x1F, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x1F, 0x02))
                }
            }

            Device (NV32)
            {
                Name (_ADR, 0x20)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x20))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x20, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x20, 0x02))
                }
            }

            Device (NV33)
            {
                Name (_ADR, 0x21)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x21))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x21, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x21, 0x02))
                }
            }

            Device (NV34)
            {
                Name (_ADR, 0x22)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x22))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x22, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x22, 0x02))
                }
            }

            Device (NV35)
            {
                Name (_ADR, 0x23)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x23))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x23, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x23, 0x02))
                }
            }

            Device (NV36)
            {
                Name (_ADR, 0x24)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x24))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x24, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x24, 0x02))
                }
            }

            Device (NV37)
            {
                Name (_ADR, 0x25)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x25))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x25, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x25, 0x02))
                }
            }

            Device (NV38)
            {
                Name (_ADR, 0x26)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x26))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x26, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x26, 0x02))
                }
            }

            Device (NV39)
            {
                Name (_ADR, 0x27)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x27))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x27, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x27, 0x02))
                }
            }

            Device (NV40)
            {
                Name (_ADR, 0x28)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x28))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x28, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x28, 0x02))
                }
            }

            Device (NV41)
            {
                Name (_ADR, 0x29)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x29))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x29, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x29, 0x02))
                }
            }

            Device (NV42)
            {
                Name (_ADR, 0x2A)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x2A))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x2A, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x2A, 0x02))
                }
            }

            Device (NV43)
            {
                Name (_ADR, 0x2B)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x2B))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x2B, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x2B, 0x02))
                }
            }

            Device (NV44)
            {
                Name (_ADR, 0x2C)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x2C))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x2C, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x2C, 0x02))
                }
            }

            Device (NV45)
            {
                Name (_ADR, 0x2D)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x2D))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x2D, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x2D, 0x02))
                }
            }

            Device (NV46)
            {
                Name (_ADR, 0x2E)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x2E))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x2E, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x2E, 0x02))
                }
            }

            Device (NV47)
            {
                Name (_ADR, 0x2F)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x2F))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x2F, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x2F, 0x02))
                }
            }

            Device (NV48)
            {
                Name (_ADR, 0x30)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x30))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x30, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x30, 0x02))
                }
            }

            Device (NV49)
            {
                Name (_ADR, 0x31)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x31))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x31, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x31, 0x02))
                }
            }

            Device (NV50)
            {
                Name (_ADR, 0x32)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x32))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x32, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x32, 0x02))
                }
            }

            Device (NV51)
            {
                Name (_ADR, 0x33)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x33))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x33, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x33, 0x02))
                }
            }

            Device (NV52)
            {
                Name (_ADR, 0x34)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x34))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x34, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x34, 0x02))
                }
            }

            Device (NV53)
            {
                Name (_ADR, 0x35)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x35))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x35, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x35, 0x02))
                }
            }

            Device (NV54)
            {
                Name (_ADR, 0x36)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x36))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x36, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x36, 0x02))
                }
            }

            Device (NV55)
            {
                Name (_ADR, 0x37)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x37))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x37, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x37, 0x02))
                }
            }

            Device (NV56)
            {
                Name (_ADR, 0x38)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x38))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x38, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x38, 0x02))
                }
            }

            Device (NV57)
            {
                Name (_ADR, 0x39)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x39))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x39, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x39, 0x02))
                }
            }

            Device (NV58)
            {
                Name (_ADR, 0x3A)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x3A))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x3A, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x3A, 0x02))
                }
            }

            Device (NV59)
            {
                Name (_ADR, 0x3B)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x3B))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x3B, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x3B, 0x02))
                }
            }

            Device (NV60)
            {
                Name (_ADR, 0x3C)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x3C))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x3C, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x3C, 0x02))
                }
            }

            Device (NV61)
            {
                Name (_ADR, 0x3D)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x3D))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x3D, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x3D, 0x02))
                }
            }

            Device (NV62)
            {
                Name (_ADR, 0x3E)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x3E))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x3E, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x3E, 0x02))
                }
            }

            Device (NV63)
            {
                Name (_ADR, 0x3F)  // _ADR: Address
                Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
                {
                    Return (DDSM (Arg0, Arg1, Arg2, Arg3, 0x3F))
                }

                Method (_NCH, 0, Serialized)  // _NCH: NVDIMM Current Health Information
                {
                    Return (DNXX (0x3F, One))
                }

                Method (_NBS, 0, Serialized)  // _NBS: NVDIMM Boot Status
                {
                    Return (DNXX (0x3F, 0x02))
                }
            }
        }

        Device (VTPM)
        {
            Name (_HID, EisaId ("VMW0004"))  // _HID: Hardware ID
            Name (_CID, Package (0x02)  // _CID: Compatible ID
            {
                "MSFT0101" /* TPM 2.0 Security Device */, 
                "ACPI\\MSFT0101"
            })
            Name (_UID, Zero)  // _UID: Unique ID
            Method (_INI, 0, Serialized)  // _INI: Initialize
            {
                If ((TOOS == 0x1000))
                {
                    _HID = "MSFT0101"
                    _CID = 0x020CD041
                }
            }

            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If (TBAS)
                {
                    Return (0x0F)
                }

                Return (Zero)
            }

            Method (_CRS, 0, Serialized)  // _CRS: Current Resource Settings
            {
                Name (TMPL, ResourceTemplate ()
                {
                    Memory32Fixed (ReadWrite,
                        0x00000000,         // Address Base
                        0x00005000,         // Address Length
                        _Y08)
                })
                CreateDWordField (TMPL, \_SB.VTPM._CRS._Y08._BAS, BASE)  // _BAS: Base Address
                BASE = TBAS /* \_SB_.TBAS */
                Return (TMPL) /* \_SB_.VTPM._CRS.TMPL */
            }

            Method (PPCK, 1, NotSerialized)
            {
                If ((Ones == Match (Package (0x09)
                            {
                                Zero, 
                                0x11, 
                                0x1A, 
                                0x1C, 
                                0x1E, 
                                0x20, 
                                0x21, 
                                0x62, 
                                0x64
                            }, MEQ, Arg0, MTR, Zero, Zero)))
                {
                    If ((Ones != Match (Package (0x07)
                                    {
                                        0x12, 
                                        0x19, 
                                        0x1B, 
                                        0x1D, 
                                        0x1F, 
                                        0x63, 
                                        0x65
                                    }, MEQ, Arg0, MTR, Zero, Zero)))
                    {
                        Return (0x03)
                    }

                    If ((Arg0 == One))
                    {
                        Local0 = TP28 /* \_SB_.TP28 */
                        Return (Zero)
                    }
                    ElseIf ((Arg0 == 0x02))
                    {
                        Local0 = TP30 /* \_SB_.TP30 */
                        Return (Zero)
                    }
                    ElseIf ((Arg0 == 0x05))
                    {
                        Local0 = TP17 /* \_SB_.TP17 */
                    }
                    ElseIf ((Arg0 == 0x17))
                    {
                        Local0 = TP26 /* \_SB_.TP26 */
                        Return (Zero)
                    }
                    ElseIf ((Arg0 == 0x18))
                    {
                        Local0 = TP32 /* \_SB_.TP32 */
                        Return (Zero)
                    }
                    ElseIf ((Arg0 == 0x22))
                    {
                        Local0 = (TP28 | TP30) /* \_SB_.TP30 */
                        Return (Zero)
                    }
                    ElseIf ((Arg0 == 0x60))
                    {
                        Local0 = TP98 /* \_SB_.TP98 */
                        Return (Zero)
                    }
                    ElseIf ((Arg0 == 0x61))
                    {
                        Local0 = T100 /* \_SB_.T100 */
                        Return (Zero)
                    }
                    ElseIf ((Ones == Match (Package (0x03)
                                {
                                    0x0E, 
                                    0x15, 
                                    0x16
                                }, MEQ, Arg0, MTR, Zero, Zero)))
                    {
                        Return (Zero)
                    }
                    Else
                    {
                        Local0 = (TP17 | TP28) /* \_SB_.TP28 */
                    }

                    If (Local0)
                    {
                        Return (0x03)
                    }
                }

                Return (0x04)
            }

            Method (PPEX, 2, Serialized)
            {
                Local0 = PPCK (Arg0)
                If ((Local0 < 0x03))
                {
                    If ((Arg0 != 0x0A))
                    {
                        Return (One)
                    }
                }

                TPRS = Arg1
                TPOP = Arg0
                TPRQ = One
                Return (Zero)
            }

            Name (BSAV, One)
            Name (BPOP, Zero)
            Name (BPRS, Zero)
            Method (_DSM, 4, Serialized)  // _DSM: Device-Specific Method
            {
                If ((Arg0 == ToUUID ("cf8e16a5-c1e8-4e25-b712-4f54a96702c8") /* Unknown UUID */))
                {
                    If ((Arg1 >= One))
                    {
                        If ((Zero == Arg2))
                        {
                            Return (Buffer (One)
                            {
                                 0x03                                             // .
                            })
                        }

                        If ((Arg2 == One))
                        {
                            Return (Package (0x02)
                            {
                                One, 
                                Package (0x02)
                                {
                                    0x02, 
                                    Zero
                                }
                            })
                        }

                        Return (Buffer (One)
                        {
                             0x00                                             // .
                        })
                    }
                }

                If ((Arg0 == ToUUID ("3dddfaa6-361b-4eb4-a424-8d10089d1653") /* Physical Presence Interface */))
                {
                    If ((Arg1 >= One))
                    {
                        If (BSAV)
                        {
                            BPOP = TPOP /* \_SB_.TPOP */
                            BPRS = TPRS /* \_SB_.TPRS */
                            BSAV = Zero
                            TPRS = Zero
                            TPOP = Zero
                        }

                        If ((Zero == Arg2))
                        {
                            Return (Buffer (0x02)
                            {
                                 0xFF, 0x01                                       // ..
                            })
                        }

                        If ((One == Arg2))
                        {
                            Return ("1.3")
                        }

                        If ((0x02 == Arg2))
                        {
                            Return (PPEX (DerefOf (Arg3 [Zero]), Zero))
                        }

                        If ((0x03 == Arg2))
                        {
                            If ((Arg1 >= 0x02))
                            {
                                Local0 = Package (0x03)
                                    {
                                        Zero, 
                                        Zero, 
                                        Zero
                                    }
                                Local0 [0x02] = TPRS /* \_SB_.TPRS */
                            }
                            Else
                            {
                                Local0 = Package (0x02)
                                    {
                                        Zero, 
                                        Zero
                                    }
                            }

                            Local0 [One] = TPOP /* \_SB_.TPOP */
                            Return (Local0)
                        }

                        If ((0x04 == Arg2))
                        {
                            Return (0x02)
                        }

                        If ((0x05 == Arg2))
                        {
                            Local0 = Package (0x03)
                                {
                                    Zero, 
                                    Zero, 
                                    Zero
                                }
                            Local0 [One] = BPOP /* \_SB_.VTPM.BPOP */
                            Local0 [0x02] = BPRS /* \_SB_.VTPM.BPRS */
                            Return (Local0)
                        }

                        If ((0x06 == Arg2))
                        {
                            Return (0x03)
                        }

                        If ((0x07 == Arg2))
                        {
                            If ((Arg1 >= 0x02))
                            {
                                Local1 = DerefOf (Arg3 [One])
                            }
                            Else
                            {
                                Local1 = Zero
                            }

                            Return (PPEX (DerefOf (Arg3 [Zero]), Local1))
                        }

                        If ((0x08 == Arg2))
                        {
                            Local0 = DerefOf (Arg3 [Zero])
                            Return (PPCK (DerefOf (Arg3 [Zero])))
                        }

                        Return (Buffer (One)
                        {
                             0x00                                             // .
                        })
                    }
                }

                If (((Arg0 == ToUUID ("376054ed-cc13-4675-901c-4756d7f2d45d") /* Unknown UUID */) && TMIM))
                {
                    If ((Arg1 >= One))
                    {
                        If ((Zero == Arg2))
                        {
                            Return (Buffer (One)
                            {
                                 0x03                                             // .
                            })
                        }

                        If ((Arg2 == One))
                        {
                            TMOR = DerefOf (Arg3 [Zero])
                            Return (Zero)
                        }

                        Return (Buffer (One)
                        {
                             0x00                                             // .
                        })
                    }
                }

                Return (Buffer (One)
                {
                     0x00                                             // .
                })
            }
        }

        Device (PCLK)
        {
            Name (_HID, EisaId ("VMW0005"))  // _HID: Hardware ID
            Name (_UID, One)  // _UID: Unique ID
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                If ((PCST & One))
                {
                    Return (0x0F)
                }

                Return (Zero)
            }
        }

        Method (L1MX, 2, NotSerialized)
        {
            Return (0xFFFFFFFE)
        }

        Processor (CP01, 0x01, 0x00000450, 0x06)
        {
            Name (CPID, 0x01)
            Method (_STA, 0, NotSerialized)  // _STA: Status
            {
                Return (CSTA (CPID))
            }

            Method (_MAT, 0, NotSerialized)  // _MAT: Multiple APIC Table Entry
            {
                Return (CMAT (CPID))
            }

            Method (_PXM, 0, NotSerialized)  // _PXM: Device Proximity
            {
                Return (CPXM (CPID))
            }
        }

        Method (CNOT, 2, NotSerialized)
        {
            If (((Arg1 < Zero) || (Arg1 >= 0x02)))
            {
                Return (0xFFFFFFFE)
            }

            If ((Arg1 < 0x01))
            {
                Notify (CP00, Arg0)
            }
            Else
            {
                Notify (CP01, Arg0)
            }

            Return (Zero)
        }

        Method (PPHR, 3, NotSerialized)
        {
            Return (^PCI0.PPHR (Arg0, Arg1, Arg2))
        }
    }

    Scope (_GPE)
    {
        Method (_L09, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            Acquire (\_SB.OEML, 0xFFFF)
            \_SB.IVOC (0x86, Zero)
            Local0 = \_SB.PCI0.OEMR
            Release (\_SB.OEML)
            If ((Local0 != Zero))
            {
                Notify (\_SB.BAT1, Local0)
                Notify (\_SB.BAT2, Local0)
                Notify (\_SB.ACAD, 0x80) // Status Change
            }

            Acquire (\_SB.OEML, 0xFFFF)
            \_SB.IVOC (0x88, Zero)
            Local0 = \_SB.PCI0.OEMR
            Release (\_SB.OEML)
            If ((Local0 & One))
            {
                Notify (\_SB.SLPB, 0x80) // Status Change
            }

            If ((Local0 & 0x02))
            {
                Notify (\_SB.SLPB, 0x02) // Device Wake
            }

            If ((Local0 != Zero))
            {
                \_SB.IVOC (0x88, (Local0 & 0xFF))
            }
        }

        Method (_L01, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            \_SB.PCI0.DVHP ()
        }

        Method (_L02, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            \_SB.DQUE ()
        }

        Method (_L03, 0, NotSerialized)  // _Lxx: Level-Triggered GPE, xx=0x00-0xFF
        {
            \_SB.AWAK ()
        }
    }

    Name (_S0, Package (0x02)  // _S0_: S0 System State
    {
        0x05, 
        0x05
    })
    Name (_S1, Package (0x02)  // _S1_: S1 System State
    {
        0x04, 
        0x04
    })
    Name (_S4, Package (0x02)  // _S4_: S4 System State
    {
        Zero, 
        Zero
    })
    Name (_S5, Package (0x02)  // _S5_: S5 System State
    {
        Zero, 
        Zero
    })
    Method (_PTS, 1, NotSerialized)  // _PTS: Prepare To Sleep
    {
        If ((Arg0 >= 0x02))
        {
            \_SB.PCI0.CRST = One
        }
        Else
        {
            \_SB.PCI0.CRST = Zero
        }
    }
}

