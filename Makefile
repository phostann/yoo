include .env

.PHONY: test
test:
	@echo "Running tests..."
	@echo ${DATABASE_URL}

.PHONY: up
up:
	sea-orm-cli migrate -u ${DATABASE_URL} --verbose

.PHONY: down
down:
	sea-orm-cli migrate down

.PHONY: create
create:
	@read -p "Enter migration name: " name; \
	sea-orm-cli migrate generate $$name -u ${DATABASE_URL}

.PHONY: entity
entity:
	sea-orm-cli generate entity -o entity/src -l --with-serde both -u ${DATABASE_URL}