#  Author:  Blair Edwards
#  Date Started:  2024-09-09

#  First version of my Rust-Library-To-C makefile; which I'm anticipating I will want to have in my back pocket.
#  Compilations we're targeting:
#   -  x86_64-unknown-linux-gnu -> sp-gtk4-loading-logos.linux.a
#   -  aarch64-unknown-linux-gnu -> sp-gtk4-loading-logos.arm.a
#   -  x86_64-pc-windows-gnu -> sp-gtk4-loading-logos.win.a

#  Project Information
LIB_NAME?=sp-gtk4-loading-logos
PATH_DEST?=.

#  Common Variables
MAKEFLAGS=-j$(shell nproc || printf 4)
GIT_HASH=$(shell git log -n 1 --pretty=format:%H)
PFX_LIB_RS=lib

#  Compiler
PATH_TGT=./target
PROFILE_REL=release-with-debug
PROFILE_DBG=debug
CMD_COMP=cargo build --lib
CMD_COMP_REL=$(CMD_COMP) --profile $(PROFILE_REL)
CMD_COMP_DBG=$(CMD_COMP)

#  Platform-Specific Variables
TAG_REL=rel
TAG_DBG=dbg
TAG_LINUX=linux
TAG_WIN=win
TAG_ARM=arm
PFX_LINUX=x86_64-unknown-linux-gnu
PFX_WIN=x86_64-pc-windows-gnu
PFX_ARM=aarch64-unknown-linux-gnu


#  High-Level Targets
all:  linux win arm
linux:  $(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_LINUX).a
win:  $(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_WIN).a
arm:  $(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_ARM).a
dbg-linux:  $(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_LINUX).a
dbg-win:  $(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_WIN).a
dbg-arm:  $(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_ARM).a

#  Cleanup
clean:
	cargo clean
	rm ./*.a


#  Pre-Calculated Strings
LIB_NAME_RS=$(subst -,_,$(LIB_NAME))
PATH_REL=$(PROFILE_REL)
PATH_DBG=$(PROFILE_DBG)
PATH_LINUX=$(PATH_TGT)/$(PFX_LINUX)
PATH_WIN=$(PATH_TGT)/$(PFX_WIN)
PATH_ARM=$(PATH_TGT)/$(PFX_ARM)


#  Compiling the target archive - release.
$(PATH_LINUX)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a:
	$(CMD_COMP_REL) --target $(PFX_LINUX)
$(PATH_WIN)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a:
	$(CMD_COMP_REL) --target $(PFX_WIN)
$(PATH_ARM)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a:
	$(CMD_COMP_REL) --target $(PFX_ARM)
#  Compiling the target archive - debug.
$(PATH_LINUX)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a:
	$(CMD_COMP_DBG) --target $(PFX_LINUX)
$(PATH_WIN)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a:
	$(CMD_COMP_DBG) --target $(PFX_WIN)
$(PATH_ARM)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a:
	$(CMD_COMP_DBG) --target $(PFX_ARM)

#  Copying the archive to the destination - release.
$(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_LINUX).a:  $(PATH_LINUX)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a
	cp $(PATH_LINUX)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a $(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_LINUX).a
$(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_WIN).a:  $(PATH_WIN)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a
	cp $(PATH_WIN)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a $(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_WIN).a
$(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_ARM).a:  $(PATH_ARM)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a
	cp $(PATH_ARM)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).a $(PATH_DEST)/$(LIB_NAME).$(TAG_REL).$(TAG_ARM).a
#  Copying the archive to the destination - debug.
$(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_LINUX).a:  $(PATH_LINUX)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a
	cp $(PATH_LINUX)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a $(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_LINUX).a
$(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_WIN).a:  $(PATH_WIN)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a
	cp $(PATH_WIN)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a $(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_WIN).a
$(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_ARM).a:  $(PATH_ARM)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a
	cp $(PATH_ARM)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).a $(PATH_DEST)/$(LIB_NAME).$(TAG_DBG).$(TAG_ARM).a


#  Remove these from implicit compilations.
.PHONY:  clean all linux win arm dbg-linux dbg-win dbg-arm
