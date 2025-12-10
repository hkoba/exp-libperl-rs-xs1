MODNAME = Mytest

BUILD = debug

all: blib/arch/auto/$(MODNAME)/$(MODNAME).so blib/lib/$(MODNAME).pm

target/$BUILD/lib$(MODNAME).so:
	cargo build

blib/arch/auto/$(MODNAME)/$(MODNAME).so: target/$(BUILD)/lib$(MODNAME).so
	test -d $(dir $@) || mkdir -p $(dir $@)
	cp -v $< $@

blib/lib/$(MODNAME).pm: perllib/$(MODNAME).pm
	test -d $(dir $@) || mkdir -p $(dir $@)
	cp -v $< $@
