// SPDX-License-Identifier: LGPL-2.1-or-later
/*
 * This file is part of libgpiod.
 *
 * Copyright (C) 2017-2018 Bartosz Golaszewski <bartekgola@gmail.com>
 */

#include <dirent.h>
#include <errno.h>
#include <getopt.h>
#include <gpiod.h>
#include <stdio.h>
#include <string.h>

#include "tools-common.h"

static const struct option longopts[] = {
	{ "help",	no_argument,	NULL,	'h' },
	{ "version",	no_argument,	NULL,	'v' },
	{ GETOPT_NULL_LONGOPT },
};

static const char *const shortopts = "+hv";

static void print_help(void)
{
	printf("Usage: %s [OPTIONS] <name>\n", get_progname());
	printf("\n");
	printf("Find a GPIO line by name. The output of this command can be used as input for gpioget/set.\n");
	printf("\n");
	printf("Options:\n");
	printf("  -h, --help:\t\tdisplay this message and exit\n");
	printf("  -v, --version:\tdisplay the version and exit\n");
}

int main(int argc, char **argv)
{
	int i, num_chips, optc, opti;
	struct gpiod_chip *chip;
	struct gpiod_line *line;
	struct dirent **entries;

	for (;;) {
		optc = getopt_long(argc, argv, shortopts, longopts, &opti);
		if (optc < 0)
			break;

		switch (optc) {
		case 'h':
			print_help();
			return EXIT_SUCCESS;
		case 'v':
			print_version();
			return EXIT_SUCCESS;
		case '?':
			die("try %s --help", get_progname());
		default:
			abort();
		}
	}

	argc -= optind;
	argv += optind;

	if (argc != 1)
		die("exactly one GPIO line name must be specified");

	num_chips = scandir("/dev/", &entries, chip_dir_filter, alphasort);
	if (num_chips < 0)
		die_perror("unable to scan /dev");

	for (i = 0; i < num_chips; i++) {
		chip = gpiod_chip_open_by_name(entries[i]->d_name);
		if (!chip) {
			if (errno == EACCES)
				continue;

			die_perror("unable to open %s", entries[i]->d_name);
		}

		line = gpiod_chip_find_line_unique(chip, argv[0]);
		if (line) {
			printf("%s %u\n",
			       gpiod_chip_name(chip), gpiod_line_offset(line));
			gpiod_chip_close(chip);
			return EXIT_SUCCESS;
		}
	}

	return EXIT_FAILURE;
}
