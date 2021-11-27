.text
.global readmem
readmem:
  ld r0, (r0)
  rts

.global writemem
writemem:
  st r1, (r0)
  rts

#define GP_LEV0 0x7e200034
#define SCALER_DISPSTAT0 0x7e400048
#define SCALER_DISPSTAT1 0x7e400058
#define SCALER_DISPSTAT2 0x7e400068

.global irq_handler
irq_handler:
  stm r0-r5,(--sp)
  mov r0, GP_LEV0
  ld r1, (r0)
  ld r2, (r0)
  ld r3, (r0)
  mov r0, SCALER_DISPSTAT0
  ld r4, (r0)
  and r4, 0x3ffff
  mul r4, (4*3)
  lea r4, outputBuffer // TODO confirm this is PIC
  st r1, (r4)
  st r2, (r4+4)
  st r3, (r4+8)
  ldm r0-r5,(sp++)
  rti

// a uint32_t[64][4096][3] array
.data
.global outputBuffer
outputBuffer:
.space 64 * 4096 * 3 * 4
