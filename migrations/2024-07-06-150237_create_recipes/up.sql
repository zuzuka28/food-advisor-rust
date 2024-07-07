-- Your SQL goes here


CREATE TABLE "categories" (
  "id" SERIAL PRIMARY KEY,
  "uuid" text UNIQUE NOT NULL,
  "name" text NOT NULL,
  "updated_at"  TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE "recipes" (
  "id" SERIAL PRIMARY KEY,
  "uuid" text UNIQUE NOT NULL,
  "cover"  bytea NOT NULL,
  "title" text NOT NULL,
  "description" text NOT NULL,
  "time_to_cook" bigint NOT NULL,
  "difficulty" text NOT NULL,
  "servings" bigint NOT NULL,
  "category_id" text NOT NULL REFERENCES categories(uuid),
  "ingredients" text[] NOT NULL,
  "guideline" text NOT NULL,
  "updated_at"  TIMESTAMP NOT NULL DEFAULT NOW(),
  "nutrients.proteins"  bigint NOT NULL,
  "nutrients.fats"  double precision NOT NULL,
  "nutrients.carbohydrates" double precision NOT NULL,
  "nutrients.fiber"  bigint NOT NULL,
  "nutrients.kcal" bigint NOT NULL
);

