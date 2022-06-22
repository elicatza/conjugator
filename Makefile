PROGRAMNAME=conjugator
LOCAL_BINDIR=target/release
PROGRAM=${LOCAL_BINDIR}/${PROGRAMNAME}

VERB_LIST=german_verb_list.txt

PREFIX=/usr/local
SHAREDIR=${PREFIX}/share/${PROGRAMNAME}
BINDIR=${PREFIX}/bin


install:
	chmod 755 $(PROGRAM)
	mkdir -p $(BINDIR)
	mkdir -p $(SHAREDIR)
	cp $(PROGRAM) $(BINDIR)/$(PROGRAMNAME)
	cp $(VERB_LIST) $(SHAREDIR)/$(VERB_LIST)

uninstall:
	rm -f $(BINDIR)/$(PROGRAMNAME)
	rm -f $(SHAREDIR)/$(VERB_LIST)


.PHONY: install uninstall 
