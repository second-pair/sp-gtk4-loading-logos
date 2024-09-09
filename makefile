#  Author:  Blair Edwards
#  Date Started:  2024-09-09

#  First version of my Rust-Library-To-C makefile; which I'm anticipating I will want to have in my back pocket.
#  Compilations we're targeting:
#   -  x86_64-unknown-linux-gnu -> sp-gtk4-loading-logos.linux.a
#   -  aarch64-unknown-linux-gnu -> sp-gtk4-loading-logos.arm.a
#   -  x86_64-pc-windows-gnu -> sp-gtk4-loading-logos.win.a

#  Project Information
LIB_NAME_RS=sp_gtk4_loading_logos
LIB_NAME_C=sp-gtk4-loading-logos
PATH_DEST=.

#  Common Variables
MAKEFLAGS=-j$(shell nproc || printf 4)
PROFILE_REL=release-with-debug
PROFILE_DBG=debug
COMP_CMD=cargo build --lib
COMP_CMD_REL=$(COMP_CMD) --profile $(PROFILE_REL)
COMP_CMD_DBG=$(COMP_CMD)
PATH_TGT=./target
PATH_REL=$(PROFILE_REL)
PATH_DBG=$(PROFILE_DBG)
PFX_LIB_RS=lib
EXT_ARCHIVE=a

#  Platform-Specific Variables
PFX_DBG=-dbg
PFX_LINUX=linux
PFX_WIN=win
PFX_ARM=arm
TRIP_LINUX=x86_64-unknown-linux-gnu
TRIP_WIN=x86_64-pc-windows-gnu
TRIP_ARM=aarch64-unknown-linux-gnu

PATH_LINUX=$(PATH_TGT)/$(TRIP_LINUX)
PATH_WIN=$(PATH_TGT)/$(TRIP_WIN)
PATH_ARM=$(PATH_TGT)/$(TRIP_ARM)

#  High-Level Targets - Release
all:  linux win arm
linux:  $(PATH_DEST)/$(LIB_NAME_C).$(PFX_LINUX).$(EXT_ARCHIVE)
win:  $(PATH_DEST)/$(LIB_NAME_C).$(PFX_WIN).$(EXT_ARCHIVE)
arm:  $(PATH_DEST)/$(LIB_NAME_C).$(PFX_ARM).$(EXT_ARCHIVE)
dbg-linux:  $(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_LINUX).$(EXT_ARCHIVE)
dbg-win:  $(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_WIN).$(EXT_ARCHIVE)
dbg-arm:  $(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_ARM).$(EXT_ARCHIVE)

#  Cleanup
clean:
	cargo clean
	rm ./*.a

#  Compiling the target archive - release.
$(PATH_LINUX)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE):
	$(COMP_CMD_REL) --target $(TRIP_LINUX)
$(PATH_WIN)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE):
	$(COMP_CMD_REL) --target $(TRIP_WIN)
$(PATH_ARM)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE):
	$(COMP_CMD_REL) --target $(TRIP_ARM)
#  Compiling the target archive - debug.
$(PATH_LINUX)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE):
	$(COMP_CMD_DBG) --target $(TRIP_LINUX)
$(PATH_WIN)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE):
	$(COMP_CMD_DBG) --target $(TRIP_WIN)
$(PATH_ARM)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE):
	$(COMP_CMD_DBG) --target $(TRIP_ARM)

#  Copying the archive to the destination - release.
$(PATH_DEST)/$(LIB_NAME_C).$(PFX_LINUX).$(EXT_ARCHIVE):  $(PATH_LINUX)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE)
	cp $(PATH_LINUX)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE) $(PATH_DEST)/$(LIB_NAME_C).$(PFX_LINUX).$(EXT_ARCHIVE)
$(PATH_DEST)/$(LIB_NAME_C).$(PFX_WIN).$(EXT_ARCHIVE):  $(PATH_WIN)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE)
	cp $(PATH_WIN)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE) $(PATH_DEST)/$(LIB_NAME_C).$(PFX_WIN).$(EXT_ARCHIVE)
$(PATH_DEST)/$(LIB_NAME_C).$(PFX_ARM).$(EXT_ARCHIVE):  $(PATH_ARM)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE)
	cp $(PATH_ARM)/$(PATH_REL)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE) $(PATH_DEST)/$(LIB_NAME_C).$(PFX_ARM).$(EXT_ARCHIVE)
#  Copying the archive to the destination - debug.
$(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_LINUX).$(EXT_ARCHIVE):  $(PATH_LINUX)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE)
	cp $(PATH_LINUX)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE) $(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_LINUX).$(EXT_ARCHIVE)
$(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_WIN).$(EXT_ARCHIVE):  $(PATH_WIN)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE)
	cp $(PATH_WIN)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE) $(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_WIN).$(EXT_ARCHIVE)
$(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_ARM).$(EXT_ARCHIVE):  $(PATH_ARM)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE)
	cp $(PATH_ARM)/$(PATH_DBG)/$(PFX_LIB_RS)$(LIB_NAME_RS).$(EXT_ARCHIVE) $(PATH_DEST)/$(LIB_NAME_C)$(PFX_DBG).$(PFX_ARM).$(EXT_ARCHIVE)

#  Remove these from implicit compilations.
.PHONY:  all linux win arm clean
