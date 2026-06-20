dev:
	cargo watch -x run --poll --why

sea-orm-codegen:
	sea-orm-cli generate entity -u sqlite://crud.db -o src/entity
