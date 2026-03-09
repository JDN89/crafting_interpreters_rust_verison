#!/usr/bin/env bash
gpg -c "Crafting Interpreters_working.pdf" &&
	mv "Crafting Interpreters_working.pdf.gpg" "Crafting Interpreters.pdf.gpg" &&
	rm "Crafting Interpreters_working.pdf"
