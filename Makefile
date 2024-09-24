QEMU = qemu-system-riscv32

QEMU_ARGS = -cpu rv32i
QEMU_ARGS += -d cpu_reset
# Must specify a machine type - from the qemu documentation
QEMU_ARGS += -machine virt
QEMU_ARGS += -vga std
QEMU_ARGS += -bios none


test: test.c
	echo "hello"

test.c:
	echo "step 1"