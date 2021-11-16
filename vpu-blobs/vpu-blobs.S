.global readmem
readmem:
  ld r0, (r0)
  rts

.global writemem
writemem:
  st r1, (r0)
  rts
