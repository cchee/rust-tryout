-- Cost Items table
CREATE TABLE IF NOT EXISTS public.cost_items (
	id bigserial NOT NULL,
	"name" varchar(64) NOT NULL,
	price numeric(10,4) NOT NULL,
	notes text NULL,
	CONSTRAINT cost_items_pk PRIMARY KEY (id),
	CONSTRAINT cost_items_un UNIQUE ("name")
);