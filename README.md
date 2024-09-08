# read-elf

## Réalisation d'un petit objdump

Lecture du ELF test/exec et renvoie :


```
Segments:
   0 PHDR           Offset = 0x00000000000040, Size = 0x000000000002d8, Flags = r--
   1 INTERP         Offset = 0x00000000000318, Size = 0x0000000000001c, Flags = r--
   2 LOAD           Offset = 0x00000000000000, Size = 0x00000000000628, Flags = r--
   3 LOAD           Offset = 0x00000000001000, Size = 0x00000000000175, Flags = r-x
   4 LOAD           Offset = 0x00000000002000, Size = 0x000000000000f4, Flags = r--
   5 LOAD           Offset = 0x00000000002db8, Size = 0x00000000000258, Flags = rw-
   6 DYNAMIC        Offset = 0x00000000002dc8, Size = 0x000000000001f0, Flags = rw-
   7 NOTE           Offset = 0x00000000000338, Size = 0x00000000000030, Flags = r--
   8 NOTE           Offset = 0x00000000000368, Size = 0x00000000000044, Flags = r--
   9 unknow         Offset = 0x00000000000338, Size = 0x00000000000030, Flags = r--
  10 unknow         Offset = 0x00000000002010, Size = 0x00000000000034, Flags = r--
  11 unknow         Offset = 0x00000000000000, Size = 0x00000000000000, Flags = rw-
  12 unknow         Offset = 0x00000000002db8, Size = 0x00000000000248, Flags = r--

Sections:
   0 <invalid name>     Offset = 0x00000000000000, Size = 0x00000000000000
                        LOAD, READONLY
   1 .interp            Offset = 0x00000000000318, Size = 0x0000000000001c
                        ALLOC, LOAD, READONLY
   2 .note.gnu.property Offset = 0x00000000000338, Size = 0x00000000000030
                        ALLOC, LOAD, READONLY
   3 .note.gnu.build-id Offset = 0x00000000000368, Size = 0x00000000000024
                        ALLOC, LOAD, READONLY
   4 .note.ABI-tag      Offset = 0x0000000000038c, Size = 0x00000000000020
                        ALLOC, LOAD, READONLY
   5 .gnu.hash          Offset = 0x000000000003b0, Size = 0x00000000000024
                        ALLOC, LOAD, READONLY
   6 .dynsym            Offset = 0x000000000003d8, Size = 0x000000000000a8
                        ALLOC, LOAD, READONLY
   7 .dynstr            Offset = 0x00000000000480, Size = 0x0000000000008d
                        ALLOC, LOAD, READONLY
   8 .gnu.version       Offset = 0x0000000000050e, Size = 0x0000000000000e
                        ALLOC, LOAD, READONLY
   9 .gnu.version_r     Offset = 0x00000000000520, Size = 0x00000000000030
                        ALLOC, LOAD, READONLY
  10 .rela.dyn          Offset = 0x00000000000550, Size = 0x000000000000c0
                        ALLOC, LOAD, READONLY
  11 .rela.plt          Offset = 0x00000000000610, Size = 0x00000000000018
                        ALLOC, LOAD, READONLY
  12 .init              Offset = 0x00000000001000, Size = 0x0000000000001b
                        ALLOC, LOAD, READONLY, CODE
  13 .plt               Offset = 0x00000000001020, Size = 0x00000000000020
                        ALLOC, LOAD, READONLY, CODE
  14 .plt.got           Offset = 0x00000000001040, Size = 0x00000000000010
                        ALLOC, LOAD, READONLY, CODE
  15 .plt.sec           Offset = 0x00000000001050, Size = 0x00000000000010
                        ALLOC, LOAD, READONLY, CODE
  16 .text              Offset = 0x00000000001060, Size = 0x00000000000107
                        ALLOC, LOAD, READONLY, CODE
  17 .fini              Offset = 0x00000000001168, Size = 0x0000000000000d
                        ALLOC, LOAD, READONLY, CODE
  18 .rodata            Offset = 0x00000000002000, Size = 0x00000000000010
                        ALLOC, LOAD, READONLY
  19 .eh_frame_hdr      Offset = 0x00000000002010, Size = 0x00000000000034
                        ALLOC, LOAD, READONLY
  20 .eh_frame          Offset = 0x00000000002048, Size = 0x000000000000ac
                        ALLOC, LOAD, READONLY
  21 .init_array        Offset = 0x00000000002db8, Size = 0x00000000000008
                        ALLOC, LOAD
  22 .fini_array        Offset = 0x00000000002dc0, Size = 0x00000000000008
                        ALLOC, LOAD
  23 .dynamic           Offset = 0x00000000002dc8, Size = 0x000000000001f0
                        ALLOC, LOAD
  24 .got               Offset = 0x00000000002fb8, Size = 0x00000000000048
                        ALLOC, LOAD
  25 .data              Offset = 0x00000000003000, Size = 0x00000000000010
                        ALLOC, LOAD
  26 .bss               Offset = 0x00000000003010, Size = 0x00000000000008
                        ALLOC
  27 .comment           Offset = 0x00000000003010, Size = 0x0000000000002b
                        READONLY
  28 .symtab            Offset = 0x00000000003040, Size = 0x00000000000360
                        READONLY
  29 .strtab            Offset = 0x000000000033a0, Size = 0x000000000001da
                        READONLY
  30 .shstrtab          Offset = 0x0000000000357a, Size = 0x0000000000011a
                        READONLY
```
