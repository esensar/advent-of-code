YEAR=$(shell sh -c 'date +"%Y"')
DAY=$(shell sh -c 'date +"%d"')

define load_input
	@echo "Loading input for $1/$2"
	@curl https://adventofcode.com/$1/day/$2/input > $1/$2/input.txt
endef

define make_day_dir
	@echo "Creating directory for $1/$2"
	@mkdir $1/$2
endef

.PHONY: load_latest
load_latest: ensure_year_dir
	@$(call make_day_dir,$(YEAR),$(DAY))
	@$(call load_input,$(YEAR),$(DAY))

.PHONY: ensure_year_dir
ensure_year_dir:
	@mkdir $(YEAR)
