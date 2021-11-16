all: vpu-blobs.bin

vpu-blobs.elf: vpu-blobs.S
	${CC} -nostdlib -nostartfiles -o $@ $^

vpu-blobs.bin: vpu-blobs.elf
	${OBJCOPY} -O binary $< $@

install: vpu-blobs.bin vpu-blobs.elf
	mkdir -pv ${out}
	cp -v $^ ${out}/
	${OBJDUMP} -t vpu-blobs.elf > ${out}/vpu-blobs.sym
