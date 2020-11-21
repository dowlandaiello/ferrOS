/*
	File: mailbox_util.s
	Author: Dowland Aiello
	Description: This file contains definitions for a variety of utility
		functions and shorthands for common operations dealing with
		mailboxes on the raspberry pi 3.

	Distribution and usage of the contents of this file are subject to the
	conditions outlined in the attached LICENSE.

	Copyright (c) 2020 Dowland Z. Aiello
*/

# Available mailbox operations and their corresponding memory sectors
.equ	MAILBOX_BASE,   0x2000B880
.equ	MAILBOX_READ,   MAILBOX_BASE
.equ	MAILBOX_STATUS, #(MAILBOX_BASE + 0x18)
.equ	MAILBOX_WRITE,  #(MAILBOX_BASE + 0x20)

# Channel IDs
.equ	FRAMEBUFFER, #1

# Status codes
.equ	FULL,  0x80000000
.equ	EMPTY, 0x40000000
.equ	LEVEL, 0x40000000

.text

/* Blocks the thread until mailbox0 is ready to write */
block_until_mailbox_writable:
	push	{r4}

	ldr	r4, =MAILBOX_STATUS
	ldr	r4, [r4]

	# Keep reading status until status is not zero
	and	r4, FULL
	cmp	r4, #0
	be	block_until_mailbox_writable

	pop	{r4} 
	ret

/* Writes the contents of register zero to the mailbox0 channel given in
	register 1 */
write_mailbox:
	bl	block_until_mailbox_writable

	ldr	r4, =MAILBOX_WRITE
	# R0 should contain the value to put in the mailbox - clear out the
	# lower 4 bits and add the channel number (stored in R1)
	and	r0, ~(0xF)
	or	r0, r1

	# Put value specified in r0 in the mailbox write sector
	str	r0, [r4]

	ret

/* Reads the contents of the mailbox0 channel given in register 0, outputting
	the read value in register 0 */
read_mailbox:
	bl	block_until_mailbox_readable

	# Read until value in mailbox0 read contains least significant 4 bits
	# that match the channel in r0
	mov	r1, #-1
	bl	read_mailbox_helper

	ret

/* Blocks the thread until mailbox0 has a value in it, as indicated by its stat
	channel */
block_until_mailbox_readable:
	ldr	r4, =MAILBOX_STATUS
	ldr	r4, [r4]

	# Wait for status to not be "EMPTY"
	and	r4, EMPTY
	cmp	r4, #0
	bne	block_until_mailbox_readable

	ret

read_mailbox_helper:
	# We'll be reading from the READ segment of the mailbox
	ldr	r1, =MAILBOX_READ

	# Ensure that the response is from the proper channel (channel in r0)
	ldr	r1, [r1]
	and	r1, r1, 0xF

	# Channels must be equal to ret
	cmp	r1, r0
	bne	read_mailbox_helper

	# Results from r1 in r0, ret
	mov	r1, r0
	ret
